use glam::Vec3;

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub point: Vec3,
    pub dir: Vec3,
}
impl Ray {
    pub fn new(point: Vec3, dir: Vec3) -> Self {
        if !dir.is_normalized() {
            panic!("dir was not normalized: {dir}");
        }
        Self { point, dir }
    }
    pub fn at(&self, t: f32) -> Vec3 {
        self.point + t * self.dir
    }
}
