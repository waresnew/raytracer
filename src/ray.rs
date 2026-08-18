use std::ops::Range;

use glam::Vec3;

use crate::{bvh::aabb::Aabb, rgb::Rgb};

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub point: Vec3,
    pub dir: Vec3,
    pub attenuation: Rgb,
}
impl Ray {
    pub fn new(point: Vec3, dir: Vec3, attenuation: Rgb) -> Self {
        if !dir.is_normalized() {
            panic!("dir was not normalized: {dir}");
        }
        if dir.is_nan() {
            panic!("dir had nan components: {dir}");
        }
        Self {
            point,
            dir,
            attenuation,
        }
    }
    pub fn at(&self, t: f32) -> Vec3 {
        self.point + t * self.dir
    }
    pub fn intersects_aabb(&self, aabb: Aabb, t_bounds: &Range<f32>) -> bool {
        let x_t1 = (aabb.min.x - self.point.x) / self.dir.x;
        let x_t2 = (aabb.max.x - self.point.x) / self.dir.x;
        let y_t1 = (aabb.min.y - self.point.y) / self.dir.y;
        let y_t2 = (aabb.max.y - self.point.y) / self.dir.y;
        let z_t1 = (aabb.min.z - self.point.z) / self.dir.z;
        let z_t2 = (aabb.max.z - self.point.z) / self.dir.z;
        let x_t_interval = (x_t1.min(x_t2), x_t1.max(x_t2));
        let y_t_interval = (y_t1.min(y_t2), y_t1.max(y_t2));
        let z_t_interval = (z_t1.min(z_t2), z_t1.max(z_t2));
        let t_min = x_t_interval.0.max(y_t_interval.0).max(z_t_interval.0);
        let t_max = x_t_interval.1.min(y_t_interval.1).min(z_t_interval.1);

        t_min <= t_max && t_max >= t_bounds.start && t_min <= t_bounds.end
    }
}
