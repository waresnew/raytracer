use glam::Vec3;

use crate::{ext::Vec3Ext, hittable::HitResult, ray::Ray, rgb::Rgb};

#[derive(Debug, Clone, Copy)]
pub struct Diffuse {
    pub colour: Rgb,
}
impl Diffuse {
    pub fn new(colour: Rgb) -> Self {
        Self { colour }
    }

    pub fn scatter_ray(&self, hit_result: HitResult) -> Option<Ray> {
        // lambertian distribution
        let dir = Self::cosine_weighted_vector(hit_result.normal);

        Some(Ray::new(
            hit_result.point,
            dir,
            self.colour * hit_result.ray.attenuation,
        ))
    }
    fn cosine_weighted_vector(normal: Vec3) -> Vec3 {
        loop {
            let rand_unit = Vec3::rand_unit();
            let cos_theta = rand_unit.dot(normal);
            if rand_unit.dot(normal) > 0.0 && rand::random_range(0.0..1.0) < cos_theta {
                return rand_unit;
            }
        }
    }
}
