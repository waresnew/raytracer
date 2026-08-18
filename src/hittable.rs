use std::ops::Range;

use glam::Vec3;

use crate::{aabb::Aabb, material::Material, ray::Ray};

pub mod parallelogram;
pub mod sphere;
pub trait Hittable {
    fn ray_hit(&self, ray: Ray, t_bounds: &Range<f32>) -> Option<HitResult>;
    fn material(&self) -> Box<dyn Material>;
    fn aabb(&self) -> Aabb;
}
pub struct HitResult {
    pub point: Vec3,
    /// convention: always points against ray_dir
    pub normal: Vec3,
    pub object: Box<dyn Hittable>,
    pub t: f32,
    pub ray: Ray,
    pub back_face: bool,
}
