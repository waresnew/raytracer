use glam::Vec3;

use crate::{ext::Vec3Ext, hittable::HitResult, material::Material, ray::Ray, rgb::Rgb};

#[derive(Debug, Clone, Copy)]
pub struct Diffuse {
    albedo: Rgb,
}
impl Diffuse {
    pub fn new(albedo: Rgb) -> Self {
        Self { albedo }
    }
}
impl Material for Diffuse {
    fn scatter_ray(&self, hit_result: &HitResult) -> Ray {
        let dir = (Vec3::rand_unit() + hit_result.normal).normalize_or(hit_result.normal); // lambertian diffuse

        Ray::new(
            hit_result.point,
            dir,
            self.albedo * hit_result.ray.attenuation,
        )
    }

    fn clone_mat(&self) -> Box<dyn Material> {
        Box::new(*self)
    }
}
