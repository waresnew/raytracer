use std::iter;

use glam::Vec2;
use image::RgbImage;
use indicatif::ProgressBar;

use crate::{
    bvh::BvhNode,
    camera::Camera,
    material::Scatter,
    ray::Ray,
    raytracer::{RaytraceConfig, RaytraceStats},
    rgb::Rgb,
};

pub struct CpuRaytracer {
    config: RaytraceConfig,
    camera: Camera,
    bvh: BvhNode,
    stats: RaytraceStats,
}
impl CpuRaytracer {
    pub fn new(camera: Camera, bvh: BvhNode, raytrace_config: RaytraceConfig) -> Self {
        Self {
            camera,
            config: raytrace_config,
            bvh,
            stats: RaytraceStats::default(),
        }
    }
    pub fn render(&mut self, progress_bar: &ProgressBar) -> (RgbImage, RaytraceStats) {
        let mut image = RgbImage::new(self.config.image_width, self.config.image_height);
        for y in 0..self.config.image_height {
            for x in 0..self.config.image_width {
                image.put_pixel(
                    x,
                    y,
                    self.calc_pixel_colour(Vec2::new(x as f32, y as f32))
                        .into_raw(),
                )
            }
            progress_bar.inc(1);
        }
        (image, self.stats)
    }
    fn calc_pixel_colour(&mut self, point: Vec2) -> Rgb {
        let sample_points: Vec<Vec2> = iter::repeat_with(|| {
            Vec2::new(
                rand::random_range(point.x - 0.5..=point.x + 0.5),
                rand::random_range(point.y - 0.5..=point.y + 0.5),
            )
        })
        .take(self.config.aa_samples as usize)
        .collect();
        let mut sum_colour = Rgb::BLACK;
        for sample_point in sample_points {
            let world_point = self.camera.screen_to_viewport(sample_point);
            let lens_point = self.camera.rand_lens_point();
            let ray_dir = world_point - lens_point;
            let ray = Ray::new(lens_point, ray_dir.normalize(), Rgb::WHITE);

            let ray_colour = self.ray_cast(ray, self.config.max_depth);
            sum_colour += ray_colour;
        }

        sum_colour / self.config.aa_samples as f32
    }

    fn ray_cast(&mut self, ray: Ray, depth: u32) -> Rgb {
        self.stats.total_rays += 1;
        if depth == 0 {
            return Rgb::BLACK;
        }
        // min=0.001 to avoid shadow acne (eg. a reflected ray may start 0.00001 inside an object bc float imprecision - want to ignore those)
        if let Some(hit_result) = self.bvh.ray_hit(ray, &(0.001..f32::INFINITY)) {
            if let Some(reflected) = hit_result.material.scatter_ray(hit_result) {
                self.ray_cast(reflected, depth - 1)
            } else {
                ray.attenuation * hit_result.material.emit_light()
            }
        } else {
            ray.attenuation * self.config.sky_colour
        }
    }
}
