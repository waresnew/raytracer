use std::ops::Range;

use glam::Vec3;

use crate::{aabb::Aabb, hittable::HitResult, material::Material, ray::Ray};

#[derive(Clone, Copy)]
pub struct Parallelogram {
    start: Vec3,
    side1: Vec3,
    side2: Vec3,
    material: Material,
}
impl Parallelogram {
    pub fn new(start: Vec3, side1: Vec3, side2: Vec3, material: Material) -> Self {
        Self {
            start,
            side1,
            side2,
            material,
        }
    }
    fn plane_normal(&self) -> Vec3 {
        self.side1.cross(self.side2).normalize()
    }
    pub fn ray_hit(&self, ray: Ray, t_bounds: &Range<f32>) -> Option<HitResult> {
        let n = self.plane_normal();
        let denom = n.dot(ray.dir);
        if denom.abs() < f32::EPSILON {
            return None; // no/inf intersection
        }
        let t = (-n.dot(ray.point) + n.dot(self.start)) / denom;
        let point = ray.at(t);
        fn point_in_parallelogram(point: Vec3, region: &Parallelogram) -> bool {
            // to check if the point is in the face's bounds, represent the point using side1 and
            // side2 as basis vectors. the basis does not (should not) need to be orthogonal and normalized.
            let d = point - region.start;
            let n = region.plane_normal();
            let s = (n.dot(region.side2.cross(d))) / (n.dot(region.side2.cross(region.side1)));
            let t = (n.dot(region.side1.cross(d))) / (n.dot(region.side1.cross(region.side2)));
            (0.0..=1.0).contains(&s) && (0.0..=1.0).contains(&t)
        }
        if t_bounds.contains(&t) && point_in_parallelogram(point, self) {
            Some(HitResult {
                point: ray.at(t),
                normal: if ray.dir.dot(n) > 0.0 { -n } else { n },
                material: self.material,
                t,
                ray,
                back_face: false,
            })
        } else {
            None
        }
    }

    pub fn aabb(&self) -> Aabb {
        const THICKNESS_PADDING: f32 = 0.0001;
        let point1 = self.start;
        let point2 = self.start + self.side1;
        let point3 = self.start + self.side2;
        let point4 = self.start + self.side1 + self.side2;
        Aabb::new(
            point1.min(point2).min(point3).min(point4),
            point1.max(point2).max(point3).max(point4) + Vec3::splat(THICKNESS_PADDING),
        )
    }
}
