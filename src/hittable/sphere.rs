use std::ops::Range;

use glam::Vec3;

use crate::{aabb::Aabb, hittable::HitResult, material::Material, ray::Ray};

#[derive(Debug, Clone, Copy)]
pub struct Sphere {
    centre: Vec3,
    radius: f32,
    material: Material,
}
impl Sphere {
    pub fn new(centre: Vec3, radius: f32, material: Material) -> Self {
        Self {
            centre,
            radius,
            material,
        }
    }
    pub fn ray_hit(&self, ray: Ray, t_bounds: &Range<f32>) -> Option<HitResult> {
        let a = ray.dir.dot(ray.dir);
        let b = 2.0 * ray.dir.dot(ray.point - self.centre);
        let c = self.centre.dot(self.centre) - 2.0 * self.centre.dot(ray.point)
            + ray.point.dot(ray.point)
            - self.radius * self.radius;
        let discrim = b * b - 4.0 * a * c;
        if discrim < 0.0 {
            return None;
        }
        let t = (-b - discrim.sqrt()) / (2.0 * a);
        if !t_bounds.contains(&t) {
            return None;
        }
        let hit_point = ray.at(t);
        let mut normal = (hit_point - self.centre).normalize();
        let mut back_face = false;
        if ray.dir.dot(normal) > 0.0 {
            normal = -normal;
            back_face = true;
        }
        Some(HitResult {
            point: ray.at(t),
            normal,
            material: self.material,
            ray,
            t,
            back_face,
        })
    }

    pub fn aabb(&self) -> Aabb {
        Aabb::new(
            self.centre - Vec3::splat(self.radius),
            self.centre + Vec3::splat(self.radius),
        )
    }
}
