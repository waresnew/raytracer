use glam::Vec3;

pub trait Vec3Ext {
    fn rand_unit() -> Vec3;
}
impl Vec3Ext for Vec3 {
    fn rand_unit() -> Vec3 {
        let mut vec = Vec3::ZERO;

        while vec.length_squared() < f32::EPSILON {
            vec = Vec3::new(
                rand::random_range(-1.0..=1.0),
                rand::random_range(-1.0..=1.0),
                rand::random_range(-1.0..=1.0),
            );
        }
        vec.normalize()
    }
}
