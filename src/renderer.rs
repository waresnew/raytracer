use glam::Vec2;
use image::RgbImage;

use crate::{camera::Camera, ray::Ray, rgb::Rgb, world::World};

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
    pub fn render_world(&self, world: &World) -> RgbImage {
        let mut image = RgbImage::new(self.width, self.height);
        for y in 0..self.height {
            for x in 0..self.width {
                image.put_pixel(
                    x,
                    y,
                    self.calc_pixel_colour(Vec2::new(x as f32, y as f32), world)
                        .into_raw(),
                )
            }
        }
        image
    }
    fn calc_pixel_colour(&self, point: Vec2, world: &World) -> Rgb {
        const AA_SAMPLES: usize = 100;
        let sample_points: [Vec2; AA_SAMPLES] = std::array::from_fn(|_| {
            Vec2::new(
                rand::random_range(point.x - 0.5..=point.x + 0.5),
                rand::random_range(point.y - 0.5..=point.y + 0.5),
            )
        });
        let mut sum_colour = Rgb::ZERO;
        for sample_point in sample_points {
            let world_point = self.screen_to_viewport(sample_point);
            let ray_dir = world_point.extend(-self.camera.focal_length) - self.camera.centre;
            let ray = Ray::new(self.camera.centre, ray_dir.normalize());

            let ray_colour = self.ray_colour(ray, world);
            sum_colour += ray_colour;
        }
        sum_colour / AA_SAMPLES as f32
    }

    fn screen_to_viewport(&self, screen: Vec2) -> Vec2 {
        let viewport = self.camera.viewport;
        let dx = viewport.dims().x / self.width as f32;
        let dy = -viewport.dims().y / self.height as f32;
        Vec2::new(viewport.min.x, viewport.max.y) + screen * Vec2::new(dx, dy)
    }
    fn ray_colour(&self, ray: Ray, world: &World) -> Rgb {
        if let Some(hit_result) = world.ray_hit(ray) {
            Rgb::from_vec3((hit_result.normal + 1.0) / 2.0 * 255.0)
        } else {
            self.ray_sky_colour(ray)
        }
    }
    fn ray_sky_colour(&self, ray: Ray) -> Rgb {
        let t = (ray.dir.y + 1.0) / 2.0;
        fn lerp(start: f32, end: f32, t: f32) -> f32 {
            (1.0 - t) * start + t * end
        }
        Rgb::new(lerp(255.0, 144.0, t), lerp(255.0, 213.0, t), 255.0)
    }
}
