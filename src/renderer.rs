use glam::Vec2;
use image::RgbImage;
use indicatif::ProgressBar;

use crate::{camera::Camera, ray::Ray, rgb::Rgb, world::World};

//TODO: most consts used here should be cli args instead
pub struct Renderer {
    pub camera: Camera,
    pub height: u32,
    pub width: u32,
}
impl Renderer {
    pub fn new(camera: Camera, height: u32, width: u32) -> Self {
        Self {
            camera,
            height,
            width,
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
    pub fn render_world(&self, world: &World, progress_bar: &ProgressBar) -> RgbImage {
        let mut image = RgbImage::new(self.width, self.height);
        for y in 0..self.height {
            for x in 0..self.width {
                image.put_pixel(
                    x,
                    y,
                    Self::linear_to_srgb(
                        self.calc_pixel_colour(Vec2::new(x as f32, y as f32), world),
                    )
                    .into_raw(),
                )
            }
            progress_bar.inc(1);
        }
        image
    }
    fn calc_pixel_colour(&self, point: Vec2, world: &World) -> Rgb {
        const AA_SAMPLES: usize = 100;
        const MAX_RAYTRACE_DEPTH: u32 = 50;
        let sample_points: [Vec2; AA_SAMPLES] = std::array::from_fn(|_| {
            Vec2::new(
                rand::random_range(point.x - 0.5..=point.x + 0.5),
                rand::random_range(point.y - 0.5..=point.y + 0.5),
            )
        });
        let mut sum_colour = Rgb::BLACK;
        for sample_point in sample_points {
            let world_point = self.camera.screen_to_viewport(sample_point);
            let lens_point = self.camera.rand_lens_point();
            let ray_dir = world_point - lens_point;
            let ray = Ray::new(lens_point, ray_dir.normalize(), Rgb::WHITE);

            let ray_colour = self.ray_cast(ray, world, MAX_RAYTRACE_DEPTH);
            sum_colour += ray_colour;
        }

        sum_colour / AA_SAMPLES as f32
    }

    fn ray_cast(&self, ray: Ray, world: &World, depth: u32) -> Rgb {
        if depth == 0 {
            return Rgb::BLACK;
        }
        if let Some(hit_result) = world.ray_hit(ray) {
            let reflected = hit_result.object.material().scatter_ray(&hit_result);
            self.ray_cast(reflected, world, depth - 1)
        } else {
            ray.attenuation * self.ray_sky_colour(ray)
        }
    }
    fn ray_sky_colour(&self, ray: Ray) -> Rgb {
        let t = (ray.dir.y + 1.0) / 2.0;
        fn lerp(start: f32, end: f32, t: f32) -> f32 {
            (1.0 - t) * start + t * end
        }
        Rgb::new(lerp(1.0, 0.5, t), lerp(1.0, 0.7, t), 1.0)
    }
}
