use std::sync::mpsc::channel;

use encase::{ShaderType, StorageBuffer, UniformBuffer};
use image::{RgbImage, imageops};
use log::info;
use wesl::include_wesl;
use wgpu::util::DeviceExt;

use crate::{
    bvh::BvhNode,
    camera::Camera,
    cpu_raytracer::RaytraceConfig,
    gpu_raytracer::structs::{
        BvhNodeGpu, CameraGpu, GpuWorkState, RaytraceConfigGpu, RgbGpu, RgbImageGpu,
    },
};

mod structs;
#[allow(dead_code)]
pub struct GpuRaytracer {
    instance: wgpu::Instance,
    adapter: wgpu::Adapter,
    device: wgpu::Device,
    queue: wgpu::Queue,
    shader: wgpu::ShaderModule,
    pipeline: wgpu::ComputePipeline,
    gpu_work_state_buffer: wgpu::Buffer,
    raytrace_config_buffer: wgpu::Buffer,
    camera_buffer: wgpu::Buffer,
    bvh_buffer: wgpu::Buffer,
    output_buffer: wgpu::Buffer,
    bind_group: wgpu::BindGroup,
    image_width: u32,
    image_height: u32,
}
const ENTRY_POINT: &str = "main";
impl GpuRaytracer {
    pub fn new(camera: Camera, bvh: BvhNode, raytrace_config: RaytraceConfig) -> Self {
        pollster::block_on(Self::new_async(camera, bvh, raytrace_config))
    }
    async fn new_async(camera: Camera, bvh: BvhNode, raytrace_config: RaytraceConfig) -> Self {
        let instance = wgpu::Instance::default();
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions::default())
            .await
            .unwrap();
        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor {
                required_limits: wgpu::Limits {
                    max_storage_buffer_binding_size: adapter
                        .limits()
                        .max_storage_buffer_binding_size,
                    max_buffer_size: adapter.limits().max_buffer_size,
                    ..Default::default()
                },
                ..Default::default()
            })
            .await
            .unwrap();
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("raytracer"),
            source: wgpu::ShaderSource::Wgsl(include_wesl!("shader").into()),
        });
        let mut gpu_work_state_uniform = UniformBuffer::new(Vec::new());
        gpu_work_state_uniform
            .write(&GpuWorkState {
                start_row: 0,
                chunk_height: 0,
            })
            .unwrap();
        let gpu_work_state_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("gpu work state"),
            contents: gpu_work_state_uniform.as_ref(),
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::UNIFORM,
        });
        let mut camera_uniform = UniformBuffer::new(Vec::new());
        let camera_gpu: CameraGpu = camera.into();
        camera_uniform.write(&camera_gpu).unwrap();
        let camera_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("camera"),
            contents: camera_uniform.as_ref(),
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::UNIFORM,
        });
        let mut raytrace_config_uniform = UniformBuffer::new(Vec::new());
        let raytrace_config_gpu: RaytraceConfigGpu = raytrace_config.into();
        raytrace_config_uniform.write(&raytrace_config_gpu).unwrap();
        let raytrace_config_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("raytrace config"),
            contents: raytrace_config_uniform.as_ref(),
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::UNIFORM,
        });
        let mut bvh_storage = StorageBuffer::new(Vec::new());
        let bvh_gpu: Vec<BvhNodeGpu> = (&bvh).into();
        bvh_storage.write(&bvh_gpu).unwrap();
        let bvh_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("bvh"),
            contents: bvh_storage.as_ref(),
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::STORAGE,
        });
        let chunk_height =
            Self::recommended_chunk_height(adapter.limits(), raytrace_config.image_width);
        let output_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("output"),
            size: raytrace_config.image_width as u64
                * chunk_height as u64
                * RgbGpu::min_size().get(),
            usage: wgpu::BufferUsages::COPY_SRC | wgpu::BufferUsages::STORAGE,
            mapped_at_creation: false,
        });
        let pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: Some("raytrace pipeline"),
            layout: None,
            module: &shader,
            entry_point: Some(ENTRY_POINT),
            compilation_options: Default::default(),
            cache: Default::default(),
        });
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &pipeline.get_bind_group_layout(0),
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: gpu_work_state_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: raytrace_config_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: camera_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 3,
                    resource: bvh_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 4,
                    resource: output_buffer.as_entire_binding(),
                },
            ],
        });

        Self {
            instance,
            adapter,
            device,
            queue,
            shader,
            pipeline,
            gpu_work_state_buffer,
            raytrace_config_buffer,
            camera_buffer,
            bvh_buffer,
            output_buffer,
            bind_group,
            image_width: raytrace_config.image_width,
            image_height: raytrace_config.image_height,
        }
    }
    pub fn render(&self) -> RgbImage {
        let chunk_height = Self::recommended_chunk_height(self.adapter.limits(), self.image_width)
            .min(self.image_height);
        info!("Using chunk height {chunk_height}");
        let mut row_start = 0;
        let mut ans = RgbImage::new(self.image_width, self.image_height);
        while row_start < self.image_height {
            let row_cnt = chunk_height.min(self.image_height - row_start);
            let chunk_res = pollster::block_on(self.render_image_chunk(
                row_start,
                row_start + row_cnt,
                0,
                self.image_width,
            ));
            imageops::replace(&mut ans, &chunk_res, 0, row_start as i64);
            row_start += row_cnt;
        }
        ans
    }
    /// [start_r,end_r)
    async fn render_image_chunk(
        &self,
        start_r: u32,
        end_r: u32,
        start_c: u32,
        end_c: u32,
    ) -> RgbImage {
        let height = end_r - start_r;
        let width = end_c - start_c;
        let mut encoder = self.device.create_command_encoder(&Default::default());
        {
            let mut cpass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: None,
                timestamp_writes: None,
            });
            cpass.set_pipeline(&self.pipeline);
            cpass.set_bind_group(0, &self.bind_group, &[]);
            cpass.dispatch_workgroups(width.div_ceil(16), height.div_ceil(16), 1);
        }
        let mut gpu_work_state_uniform = UniformBuffer::new(Vec::new());
        gpu_work_state_uniform
            .write(&GpuWorkState {
                start_row: start_r,
                chunk_height: height,
            })
            .unwrap();
        self.queue.write_buffer(
            &self.gpu_work_state_buffer,
            0,
            gpu_work_state_uniform.as_ref(),
        );
        self.queue.submit(Some(encoder.finish()));
        let (tx, rx) = channel();
        wgpu::util::DownloadBuffer::read_buffer(
            &self.device,
            &self.queue,
            &self.output_buffer.slice(..),
            move |result| tx.send(result.unwrap().to_vec()).unwrap(),
        );
        self.device
            .poll(wgpu::PollType::wait_indefinitely())
            .unwrap();

        let bytes = rx.recv().unwrap();
        let output_storage = StorageBuffer::new(bytes);
        let mut output_image_buffer: Vec<RgbGpu> = Vec::new();
        output_storage.read(&mut output_image_buffer).unwrap();
        let output_image = RgbImageGpu {
            height,
            width,
            buffer: output_image_buffer,
        };
        output_image.into()
    }
    //TODO: add cli arg to configure chunk height
    fn recommended_chunk_height(limits: wgpu::Limits, image_width: u32) -> u32 {
        let max_size = limits
            .max_storage_buffer_binding_size
            .min(limits.max_buffer_size)
            .min(u32::MAX as u64) as u32;
        (max_size / image_width / RgbGpu::min_size().get() as u32).max(1)
    }
}
