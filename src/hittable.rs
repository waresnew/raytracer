use glam::Vec3;

use crate::ray::Ray;

pub mod sphere;
pub trait Hittable {
    fn ray_hit(&self, ray: Ray) -> Option<HitResult>;
    fn centre(&self) -> Vec3;
}
#[derive(Debug, Clone, Copy)]
pub struct HitResult {
    pub point: Vec3,
    pub normal: Vec3,
}
