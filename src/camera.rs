use glam::{Vec2, Vec3};

use crate::ext::Vec2Ext;

/// +x is right, +y is up, +z is out of screen
#[derive(Debug, Clone, Copy)]
pub struct Camera {
    pub image_dims: Vec2,
    pub viewport_dims: Vec2,
    pub config: CameraConfig,
    pub basis: (Vec3, Vec3, Vec3),
}
#[derive(Debug, Clone, Copy)]
pub struct CameraConfig {
    pub centre: Vec3,
    /// this determines focus distance for now
    pub look_at_centre: Vec3,
    pub vertical_fov: f32,
    pub lens_radius: f32,
}
impl Camera {
    pub fn new(image_dims: Vec2, config: CameraConfig) -> Self {
        if config.lens_radius < 0.0 {
            panic!("lens_radius was negative: {}", config.lens_radius);
        }
        let global_up = Vec3::new(0.0, 1.0, 0.0);
        let z = (config.centre - config.look_at_centre).normalize();
        if global_up.cross(z).length() < f32::EPSILON {
            panic!("do not use completely vertical camera views");
        }
        let x = global_up.cross(z).normalize();
        let y = z.cross(x).normalize();

        let focal_length = (config.centre - config.look_at_centre).length();
        let viewport_height = 2.0 * (config.vertical_fov.to_radians() / 2.0).tan() * focal_length;
        let aspect_ratio = image_dims.x / image_dims.y;

        Self {
            viewport_dims: Vec2::new(viewport_height * aspect_ratio, viewport_height),
            image_dims,
            basis: (x, y, z),
            config,
        }
    }
    pub fn screen_to_viewport(&self, screen: Vec2) -> Vec3 {
        let viewport_min = self.config.look_at_centre - self.viewport_dims.x / 2.0 * self.basis.0
            + self.viewport_dims.y / 2.0 * self.basis.1;
        let dx = self.viewport_dims.x / self.image_dims.x;
        let dy = -self.viewport_dims.y / self.image_dims.y;
        let x_delta = dx * self.basis.0;
        let y_delta = dy * self.basis.1;
        viewport_min + screen.x * x_delta + screen.y * y_delta
    }
    pub fn focal_length(&self) -> f32 {
        (self.config.look_at_centre - self.config.centre).length()
    }
    pub fn rand_lens_point(&self) -> Vec3 {
        let rand_unit = Vec2::rand_unit();
        self.config.lens_radius * rand_unit.y * self.basis.0
            + self.config.lens_radius * rand_unit.y * self.basis.1
            + self.config.centre
    }
}
