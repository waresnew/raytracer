use std::ops::Range;

use glam::Vec3;

use crate::{
    bvh::aabb::Aabb,
    hittable::{HitResult, Hittable},
    material::Material,
    ray::Ray,
};

pub struct Sphere {
    pub centre: Vec3,
    pub radius: f32,
    pub material: Box<dyn Material>,
}
impl Sphere {
    pub fn new<M: Material + 'static>(centre: Vec3, radius: f32, material: M) -> Self {
        Self {
            centre,
            radius,
            material: Box::new(material),
        }
    }
}
impl Clone for Sphere {
    fn clone(&self) -> Self {
        Self {
            centre: self.centre,
            radius: self.radius,
            material: (*self.material).clone_mat(),
        }
    }
}
impl Hittable for Sphere {
    fn ray_hit(&self, ray: Ray, t_bounds: &Range<f32>) -> Option<HitResult> {
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
        let normal = (hit_point - self.centre()).normalize();
        Some(HitResult {
            point: ray.at(t),
            normal,
            object: Box::new(self.clone()),
            ray,
            t,
        })
    }

    fn centre(&self) -> Vec3 {
        self.centre
    }

    fn material(&self) -> Box<dyn Material> {
        self.material.clone_mat()
    }
    fn aabb(&self) -> Aabb {
        Aabb::new(
            self.centre - Vec3::splat(self.radius),
            self.centre + Vec3::splat(self.radius),
        )
    }
}
