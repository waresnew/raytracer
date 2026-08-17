use glam::Vec3;

use crate::{ext::Vec3Ext, hittable::HitResult, material::Material, ray::Ray, rgb::Rgb};

#[derive(Debug, Clone, Copy)]
pub struct Metal {
    albedo: Rgb,
    fuzz: f32,
}
impl Metal {
    pub fn new(albedo: Rgb, fuzz: f32) -> Self {
        Self { albedo, fuzz }
    }
}
impl Material for Metal {
    fn scatter_ray(&self, hit_result: &HitResult) -> Ray {
        let reflected = hit_result.ray.dir.reflect(hit_result.normal);
        let dir = (reflected + Vec3::rand_unit() * self.fuzz).normalize_or(reflected);
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
