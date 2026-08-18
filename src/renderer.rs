use std::iter;

use glam::Vec2;
use image::RgbImage;
use indicatif::ProgressBar;

use crate::{
    bvh::BvhNode, camera::Camera, hittable::HitResult, material::Material, ray::Ray, rgb::Rgb,
};

pub struct Renderer {
    pub height: u32,
    pub width: u32,
    pub config: RenderConfig,
    pub camera: Camera,
}
#[derive(Debug, Clone, Copy)]
pub struct RenderConfig {
    pub aa_samples: usize,
    pub max_depth: u32,
    pub sky_colour: Rgb,
}
impl Renderer {
    pub fn new(height: u32, width: u32, camera: Camera, config: RenderConfig) -> Self {
        Self {
            height,
            width,
            config,
            camera,
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
    pub fn render(&self, bvh: &BvhNode, progress_bar: &ProgressBar) -> RgbImage {
        let mut image = RgbImage::new(self.width, self.height);
        for y in 0..self.height {
            for x in 0..self.width {
                image.put_pixel(
                    x,
                    y,
                    Self::linear_to_srgb(
                        self.calc_pixel_colour(Vec2::new(x as f32, y as f32), bvh),
                    )
                    .into_raw(),
                )
            }
            progress_bar.inc(1);
        }
        image
    }
    fn calc_pixel_colour(&self, point: Vec2, bvh: &BvhNode) -> Rgb {
        let sample_points: Vec<Vec2> = iter::repeat_with(|| {
            Vec2::new(
                rand::random_range(point.x - 0.5..=point.x + 0.5),
                rand::random_range(point.y - 0.5..=point.y + 0.5),
            )
        })
        .take(self.config.aa_samples)
        .collect();
        let mut sum_colour = Rgb::BLACK;
        for sample_point in sample_points {
            let world_point = self.camera.screen_to_viewport(sample_point);
            let lens_point = self.camera.rand_lens_point();
            let ray_dir = world_point - lens_point;
            let ray = Ray::new(lens_point, ray_dir.normalize(), Rgb::WHITE);

            let ray_colour = self.ray_cast(ray, bvh, self.config.max_depth);
            sum_colour += ray_colour;
        }

        sum_colour / self.config.aa_samples as f32
    }

    fn ray_cast(&self, ray: Ray, bvh: &BvhNode, depth: u32) -> Rgb {
        if depth == 0 {
            return Rgb::BLACK;
        }
        // min=0.001 to avoid shadow acne (eg. a reflected ray may start 0.00001 inside an object bc float imprecision - want to ignore those)
        if let Some(hit_result) = bvh.ray_hit(ray, &(0.001..f32::INFINITY)) {
            if let Some(reflected) = Self::material_scatter_ray(hit_result.material, hit_result) {
                self.ray_cast(reflected, bvh, depth - 1)
            } else {
                ray.attenuation
                    * Self::material_emit_light(hit_result.material).unwrap_or(Rgb::BLACK)
            }
        } else {
            ray.attenuation * self.config.sky_colour
        }
    }
    fn material_scatter_ray(material: Material, hit_result: HitResult) -> Option<Ray> {
        match material {
            Material::Diffuse(diffuse) => diffuse.scatter_ray(hit_result),
            Material::Metal(metal) => metal.scatter_ray(hit_result),
            Material::Glass(glass) => glass.scatter_ray(hit_result),
            Material::DiffuseLight(_) => None,
        }
    }
    fn material_emit_light(material: Material) -> Option<Rgb> {
        match material {
            Material::Diffuse(_) => None,
            Material::Metal(_) => None,
            Material::Glass(_) => None,
            Material::DiffuseLight(diffuse_light) => Some(diffuse_light.emit_light()),
        }
    }
}
