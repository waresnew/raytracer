use std::ops::Range;

use glam::Vec3;

use crate::{bvh::aabb::Aabb, material::Material, ray::Ray};

pub mod sphere;
pub trait Hittable {
    fn ray_hit(&self, ray: Ray, t_bounds: &Range<f32>) -> Option<HitResult>;
    fn centre(&self) -> Vec3;
    fn material(&self) -> Box<dyn Material>;
    fn aabb(&self) -> Aabb;
}
pub struct HitResult {
    pub point: Vec3,
    pub normal: Vec3,
    pub object: Box<dyn Hittable>,
    pub t: f32,
    pub ray: Ray,
}
