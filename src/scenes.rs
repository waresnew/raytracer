use crate::{
    camera::CameraConfig,
    hittable::{Hittable, parallelogram::Parallelogram},
    material::Material,
    raytracer::RaytracerConfig,
    scenes::{
        cornell_box::load_cornell_box, mixed_light::load_mixed_light,
        random_balls::load_random_balls,
    },
};
use clap::ValueEnum;
use glam::Vec3;

mod cornell_box;
mod mixed_light;
mod random_balls;
#[derive(Default, Debug, Clone, Copy, ValueEnum)]
pub enum SceneType {
    #[default]
    CornellBox,
    RandomBalls,
    MixedLight,
}
pub struct Scene {
    pub objects: Vec<Hittable>,
    pub render_config: RaytracerConfig,
    pub camera_config: CameraConfig,
}

pub fn load_scene(scene: SceneType) -> Scene {
    match scene {
        SceneType::CornellBox => load_cornell_box(),
        SceneType::RandomBalls => load_random_balls(),
        SceneType::MixedLight => load_mixed_light(),
    }
}
/// yaw in degrees, +ve means ccw
pub(super) fn make_box(material: Material, centre: Vec3, yaw: f32, scale: Vec3) -> [Hittable; 6] {
    let mut base = [
        // bottom face
        (
            Vec3::new(-0.5, -0.5, -0.5),
            Vec3::new(1.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, 1.0),
        ),
        // top face
        (
            Vec3::new(-0.5, 0.5, -0.5),
            Vec3::new(1.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, 1.0),
        ),
        // left face
        (
            Vec3::new(-0.5, -0.5, -0.5),
            Vec3::new(0.0, 1.0, 0.0),
            Vec3::new(0.0, 0.0, 1.0),
        ),
        // right face
        (
            Vec3::new(0.5, -0.5, -0.5),
            Vec3::new(0.0, 1.0, 0.0),
            Vec3::new(0.0, 0.0, 1.0),
        ),
        // front face
        (
            Vec3::new(-0.5, -0.5, 0.5),
            Vec3::new(1.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
        ),
        // back face
        (
            Vec3::new(-0.5, -0.5, -0.5),
            Vec3::new(1.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
        ),
    ];
    for (p1, p2, p3) in &mut base {
        *p1 *= scale;
        *p2 *= scale;
        *p3 *= scale;

        *p1 = p1.rotate_y(yaw.to_radians());
        *p2 = p2.rotate_y(yaw.to_radians());
        *p3 = p3.rotate_y(yaw.to_radians());

        *p1 += centre;
    }
    base.map(|(start, side1, side2)| {
        Hittable::Parallelogram(Parallelogram::new(start, side1, side2, material))
    })
}
