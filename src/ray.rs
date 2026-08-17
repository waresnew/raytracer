use glam::Vec3;

use crate::rgb::Rgb;

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
}
