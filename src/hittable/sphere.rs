use glam::Vec3;

use crate::{
    hittable::{HitResult, Hittable},
    ray::Ray,
};

pub struct Sphere {
    pub centre: Vec3,
    pub radius: f32,
}
impl Sphere {
    pub fn new(centre: Vec3, radius: f32) -> Self {
        Self { centre, radius }
    }
}
impl Hittable for Sphere {
    fn ray_hit(&self, ray: Ray) -> Option<HitResult> {
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
        if t < 0.0 {
            return None;
        }
        let hit_point = ray.at(t);
        let normal = (hit_point - self.centre()).normalize();
        Some(HitResult {
            point: ray.at(t),
            normal,
        })
    }

    fn centre(&self) -> Vec3 {
        self.centre
    }
}
