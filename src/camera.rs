use glam::Vec3;

use crate::viewport::Viewport;

/// +x is right, +y is up, +z is out of screen
pub struct Camera {
    pub centre: Vec3,
    pub viewport: Viewport,
    pub focal_length: f32,
}
