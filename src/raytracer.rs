use glam::Vec2;
use image::{ConvertColorOptions, RgbImage, metadata::Cicp};
use indicatif::ProgressBar;

use crate::{
    bvh::BvhNode, camera::Camera, cpu_raytracer::CpuRaytracer, gpu_raytracer::GpuRaytracer,
    scenes::Scene,
};

pub struct RaytracerFacade {
    cpu: Option<CpuRaytracer>,
    gpu: Option<GpuRaytracer>,
}
impl RaytracerFacade {
    pub fn new(cpu: bool, scene: Scene) -> Self {
        let camera = Camera::new(
            Vec2::new(
                scene.raytrace_config.image_width as f32,
                scene.raytrace_config.image_height as f32,
            ),
            scene.camera_config,
        );
        let bvh = BvhNode::from_objects(scene.objects);
        if cpu {
            Self {
                cpu: Some(CpuRaytracer::new(camera, bvh, scene.raytrace_config)),
                gpu: None,
            }
        } else {
            Self {
                cpu: None,
                gpu: Some(GpuRaytracer::new(camera, bvh, scene.raytrace_config)),
            }
        }
    }
    pub fn render(self, progress_bar: &ProgressBar) -> RgbImage {
        let mut image = if let Some(cpu) = self.cpu {
            cpu.render(progress_bar)
        } else if let Some(gpu) = self.gpu {
            gpu.render()
        } else {
            unreachable!("neither cpu nor gpu were initialized");
        };
        image.set_color_space(Cicp::SRGB_LINEAR).unwrap();
        image
            .apply_color_space(Cicp::SRGB, ConvertColorOptions::default())
            .unwrap();
        image
    }
}
