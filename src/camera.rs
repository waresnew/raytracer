use glam::{Vec2, Vec3};

/// +x is right, +y is up, +z is out of screen
pub struct Camera {
    centre: Vec3,
    viewport_dims: Vec2,
    image_dims: Vec2,

    /// this determines focus distance for now
    look_at_centre: Vec3,

    basis: (Vec3, Vec3, Vec3),
    lens_radius: f32,
}
impl Camera {
    pub fn new(
        centre: Vec3,
        look_at_centre: Vec3,
        image_dims: Vec2,
        vertical_fov: f32,
        lens_radius: f32,
    ) -> Self {
        if lens_radius < 0.0 {
            panic!("lens_radius was negative: {}", lens_radius);
        }
        let global_up = Vec3::new(0.0, 1.0, 0.0);
        let z = (centre - look_at_centre).normalize();
        let x = global_up.cross(z).normalize();
        let y = z.cross(x).normalize();

        let focal_length = (centre - look_at_centre).length();
        let viewport_height = 2.0 * (vertical_fov.to_radians() / 2.0).tan() * focal_length;
        let aspect_ratio = image_dims.x / image_dims.y;

        Self {
            centre,
            viewport_dims: Vec2::new(viewport_height * aspect_ratio, viewport_height),
            look_at_centre,
            image_dims,
            basis: (x, y, z),
            lens_radius,
        }
    }
    pub fn screen_to_viewport(&self, screen: Vec2) -> Vec3 {
        let viewport_min = self.look_at_centre - self.viewport_dims.x / 2.0 * self.basis.0
            + self.viewport_dims.y / 2.0 * self.basis.1;
        let dx = self.viewport_dims.x / self.image_dims.x;
        let dy = -self.viewport_dims.y / self.image_dims.y;
        let x_delta = dx * self.basis.0;
        let y_delta = dy * self.basis.1;
        viewport_min + screen.x * x_delta + screen.y * y_delta
    }
    pub fn focal_length(&self) -> f32 {
        (self.look_at_centre - self.centre).length()
    }
    pub fn rand_lens_point(&self) -> Vec3 {
        let rand_x = rand::random_range(-self.lens_radius..=self.lens_radius);
        let rand_y = rand::random_range(-self.lens_radius..=self.lens_radius);
        rand_x * self.basis.0 + rand_y * self.basis.1 + self.centre
    }
}
