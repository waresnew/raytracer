use std::ops::Range;

use glam::Vec3;

use crate::ray::Ray;
#[derive(Clone, Copy, Debug)]
pub struct Aabb {
    pub min: Vec3,
    pub max: Vec3,
}
impl Aabb {
    pub fn empty_box() -> Self {
        Self {
            min: Vec3::ZERO,
            max: Vec3::ZERO,
        }
    }
    pub fn new(a: Vec3, b: Vec3) -> Self {
        Self {
            min: a.min(b),
            max: a.max(b),
        }
    }
    pub fn combine(a: Aabb, b: Aabb) -> Self {
        Self {
            min: a.min.min(b.min),
            max: a.max.max(b.max),
        }
    }
    pub fn ray_intersects(&self, ray: Ray, t_bounds: &Range<f32>) -> bool {
        let x_t1 = (self.min.x - ray.point.x) / ray.dir.x;
        let x_t2 = (self.max.x - ray.point.x) / ray.dir.x;
        let y_t1 = (self.min.y - ray.point.y) / ray.dir.y;
        let y_t2 = (self.max.y - ray.point.y) / ray.dir.y;
        let z_t1 = (self.min.z - ray.point.z) / ray.dir.z;
        let z_t2 = (self.max.z - ray.point.z) / ray.dir.z;
        let x_t_interval = (x_t1.min(x_t2), x_t1.max(x_t2));
        let y_t_interval = (y_t1.min(y_t2), y_t1.max(y_t2));
        let z_t_interval = (z_t1.min(z_t2), z_t1.max(z_t2));
        let t_min = x_t_interval.0.max(y_t_interval.0).max(z_t_interval.0);
        let t_max = x_t_interval.1.min(y_t_interval.1).min(z_t_interval.1);

        t_min <= t_max && t_max >= t_bounds.start && t_min <= t_bounds.end
    }
    pub fn max_range_axis(&self) -> usize {
        let x = self.max.x - self.min.x;
        let y = self.max.y - self.min.y;
        let z = self.max.z - self.min.z;
        if x > y && x > z {
            0
        } else if y > x && y > z {
            1
        } else {
            2
        }
    }
}
