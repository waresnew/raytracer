use glam::Vec3;

pub trait Vec3Ext {
    fn rand_unit() -> Vec3;
}
impl Vec3Ext for Vec3 {
    fn rand_unit() -> Vec3 {
        let mut vec = Vec3::ZERO;

        while vec.length_squared() < f32::EPSILON || vec.length_squared() > 1.0 {
            vec = Vec3::new(
                rand::random_range(-1.0..=1.0),
                rand::random_range(-1.0..=1.0),
                rand::random_range(-1.0..=1.0),
            );
        }
        vec.normalize()
    }
}
use glam::Vec2;

pub trait Vec2Ext {
    fn rand_unit() -> Vec2;
}
impl Vec2Ext for Vec2 {
    fn rand_unit() -> Vec2 {
        let mut vec = Vec2::ZERO;

        while vec.length_squared() < f32::EPSILON || vec.length_squared() > 1.0 {
            vec = Vec2::new(
                rand::random_range(-1.0..=1.0),
                rand::random_range(-1.0..=1.0),
            );
        }
        vec.normalize()
    }
}
