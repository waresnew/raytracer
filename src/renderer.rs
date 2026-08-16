use glam::Vec2;
use image::{Rgb, RgbImage};

use crate::{camera::Camera, ray::Ray};

pub struct Renderer {
    pub camera: Camera,
    pub height: u32,
    pub width: u32,
}
impl Renderer {
    pub fn render_to_image(&self) -> RgbImage {
        let mut image = RgbImage::new(self.width, self.height);
        for y in 0..self.height {
            for x in 0..self.width {
                let world_point = self.screen_to_viewport(Vec2::new(x as f32, y as f32));
                let ray_dir = world_point.extend(-self.camera.focal_length) - self.camera.centre;
                let ray = Ray::new(self.camera.centre, ray_dir.normalize());
                image.put_pixel(x, y, self.ray_colour(ray))
            }
        }
        image
    }
    pub fn screen_to_viewport(&self, screen: Vec2) -> Vec2 {
        let viewport = self.camera.viewport;
        let dx = viewport.dims().x / self.width as f32;
        let dy = -viewport.dims().y / self.height as f32;
        Vec2::new(viewport.min.x, viewport.max.y) + screen * Vec2::new(dx, dy)
    }
    pub fn ray_colour(&self, ray: Ray) -> Rgb<u8> {
        let t = (ray.dir.y + 1.0) / 2.0;
        fn lerp(start: u8, end: u8, t: f32) -> u8 {
            ((1.0 - t) * start as f32 + t * end as f32) as u8
        }
        Rgb([lerp(255, 144, t), lerp(255, 213, t), 255])
    }
}
