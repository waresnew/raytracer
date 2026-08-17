use std::ops::Range;

use glam::Vec3;

use crate::ray::Ray;

pub mod sphere;
pub trait Hittable {
    fn ray_hit(&self, ray: Ray, t_bounds: Range<f32>) -> Option<HitResult>;
    fn centre(&self) -> Vec3;
    fn reflect_ray(&self, hit_result: &HitResult) -> Ray;
}
pub struct HitResult {
    pub point: Vec3,
    pub normal: Vec3,
    pub object: Box<dyn Hittable>,
    pub t: f32,
}
