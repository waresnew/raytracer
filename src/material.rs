use crate::{hittable::HitResult, ray::Ray, rgb::Rgb};

pub mod diffuse;
pub mod metal;
pub trait Material {
    fn scatter_ray(&self, hit_result: &HitResult) -> Ray;
    fn clone_mat(&self) -> Box<dyn Material>;
}
