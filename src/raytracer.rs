use glam::Vec2;
use image::{ConvertColorOptions, RgbImage, metadata::Cicp};
use indicatif::{ProgressBar, ProgressStyle};

use crate::{
    bvh::BvhNode, camera::Camera, raytracer::cpu::CpuRaytracer, raytracer::gpu::GpuRaytracer,
    rgb::Rgb, scenes::Scene,
};
mod cpu;
mod gpu;

#[derive(Debug, Clone, Copy)]
pub struct RaytraceConfig {
    pub image_height: u32,
    pub image_width: u32,
    pub aa_samples: u32,
    pub max_depth: u32,
    pub sky_colour: Rgb,
}
#[derive(Debug, Clone, Copy, Default)]
pub struct RaytraceStats {
    pub total_rays: u64,
}
pub struct RaytracerFacade {
    cpu: Option<CpuRaytracer>,
    gpu: Option<GpuRaytracer>,
    progress_bar: ProgressBar,
}
impl RaytracerFacade {
    pub fn new(cpu: bool, scene: Scene, chunk_height: Option<u32>) -> Self {
        let camera = Camera::new(
            Vec2::new(
                scene.raytrace_config.image_width as f32,
                scene.raytrace_config.image_height as f32,
            ),
            scene.camera_config,
        );
        let progress_bar = ProgressBar::new(scene.raytrace_config.image_height as u64).with_style(
            ProgressStyle::with_template(
                "\t[{elapsed_precise}] {wide_bar:.green/red} {pos:>7}/{len:7} ETA: {eta}\t",
            )
            .unwrap(),
        );
        let bvh = BvhNode::from_objects(scene.objects);
        if cpu {
            Self {
                cpu: Some(CpuRaytracer::new(camera, bvh, scene.raytrace_config)),
                gpu: None,
                progress_bar: progress_bar.clone(),
            }
        } else {
            Self {
                cpu: None,
                gpu: Some(GpuRaytracer::new(
                    camera,
                    bvh,
                    scene.raytrace_config,
                    chunk_height,
                )),
                progress_bar: progress_bar.clone(),
            }
        }
    }
    pub fn render(self) -> (RgbImage, RaytraceStats) {
        let (mut image, stats) = if let Some(mut cpu) = self.cpu {
            cpu.render(&self.progress_bar)
        } else if let Some(gpu) = self.gpu {
            gpu.render(&self.progress_bar)
        } else {
            unreachable!("neither cpu nor gpu were initialized");
        };
        image.set_color_space(Cicp::SRGB_LINEAR).unwrap();
        image
            .apply_color_space(Cicp::SRGB, ConvertColorOptions::default())
            .unwrap();
        (image, stats)
    }
    pub fn progress_bar(&self) -> &ProgressBar {
        &self.progress_bar
    }
}
