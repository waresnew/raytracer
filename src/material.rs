use crate::{hittable::HitResult, ray::Ray, rgb::Rgb};

pub mod diffuse;
pub mod diffuse_light;
pub mod glass;
pub mod metal;
pub trait Material {
    fn scatter_ray(&self, hit_result: &HitResult) -> Option<Ray>;
    fn clone_mat(&self) -> Box<dyn Material>;
    fn emit_light(&self) -> Rgb {
        Rgb::BLACK
    }
}
