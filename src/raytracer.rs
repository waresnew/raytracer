use std::iter;

use glam::Vec2;
use image::RgbImage;
use indicatif::ProgressBar;

use crate::{bvh::BvhNode, camera::Camera, material::Scatter, ray::Ray, rgb::Rgb, scenes::Scene};

pub struct Raytracer {
    config: RaytraceConfig,
    camera: Camera,
    bvh: BvhNode,
}
#[derive(Debug, Clone, Copy)]
pub struct RaytraceConfig {
    pub image_height: u32,
    pub image_width: u32,
    pub aa_samples: u32,
    pub max_depth: u32,
    pub sky_colour: Rgb,
}
impl Raytracer {
    pub fn new(scene: Scene) -> Self {
        let camera = Camera::new(
            Vec2::new(
                scene.raytrace_config.image_width as f32,
                scene.raytrace_config.image_height as f32,
            ),
            scene.camera_config,
        );
        let bvh = BvhNode::from_objects(scene.objects);
        Self {
            camera,
            config: scene.raytrace_config,
            bvh,
        }
    }
    fn linear_to_srgb(rgb: Rgb) -> Rgb {
        //approximation
        fn sqrt_or_zero(x: f32) -> f32 {
            if x < 0.0 { 0.0 } else { x.sqrt() }
        }
        Rgb {
            r: sqrt_or_zero(rgb.r),
            g: sqrt_or_zero(rgb.g),
            b: sqrt_or_zero(rgb.b),
        }
    }
    pub fn render(&self, progress_bar: &ProgressBar) -> RgbImage {
        let mut image = RgbImage::new(self.config.image_width, self.config.image_height);
        for y in 0..self.config.image_height {
            for x in 0..self.config.image_width {
                image.put_pixel(
                    x,
                    y,
                    Self::linear_to_srgb(self.calc_pixel_colour(Vec2::new(x as f32, y as f32)))
                        .into_raw(),
                )
            }
            progress_bar.inc(1);
        }
        image
    }
    fn calc_pixel_colour(&self, point: Vec2) -> Rgb {
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

    fn ray_cast(&self, ray: Ray, depth: u32) -> Rgb {
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
