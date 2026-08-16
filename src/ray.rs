use glam::Vec3;

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub origin: Vec3,
    pub dir: Vec3,
}
impl Ray {
    pub fn new(origin: Vec3, dir: Vec3) -> Self {
        if !dir.is_normalized() {
            panic!("dir was not normalized: {dir}");
        }
        Self { origin, dir }
    }
    pub fn at(&self, t: f32) -> Vec3 {
        self.origin + t * self.dir
    }
}
