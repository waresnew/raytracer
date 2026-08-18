use glam::Vec3;

use crate::{
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
