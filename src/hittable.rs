use std::ops::Range;

use glam::Vec3;

use crate::{
    aabb::Aabb,
    hittable::{parallelogram::Parallelogram, sphere::Sphere},
    material::Material,
    ray::Ray,
};

pub mod parallelogram;
pub mod sphere;
#[derive(Clone, Copy)]
pub enum Hittable {
    Sphere(Sphere),
    Parallelogram(Parallelogram),
}
pub trait Hit {
    fn ray_hit(&self, ray: Ray, t_bounds: &Range<f32>) -> Option<HitResult>;
    fn aabb(&self) -> Aabb;
}
impl Hit for Hittable {
    fn ray_hit(&self, ray: Ray, t_bounds: &Range<f32>) -> Option<HitResult> {
        match self {
            Hittable::Sphere(sphere) => sphere.ray_hit(ray, t_bounds),
            Hittable::Parallelogram(parallelogram) => parallelogram.ray_hit(ray, t_bounds),
        }
    }

    fn aabb(&self) -> Aabb {
        match self {
            Hittable::Sphere(sphere) => sphere.aabb(),
            Hittable::Parallelogram(parallelogram) => parallelogram.aabb(),
        }
    }
}
#[derive(Clone, Copy)]
pub struct HitResult {
    pub point: Vec3,
    /// convention: always points against ray_dir
    pub normal: Vec3,
    pub material: Material,
    pub t: f32,
    pub ray: Ray,
    pub back_face: bool,
}
