use crate::{
    camera::CameraConfig,
    hittable::{Hittable, parallelogram::Parallelogram},
    material::{Material, diffuse::Diffuse, diffuse_light::DiffuseLight},
    raytracer::RaytraceConfig,
    rgb::Rgb,
    scenes::{
        cornell_box::load_cornell_box, cornell_box_glass::load_cornell_box_glass,
        mixed_light::load_mixed_light, random_balls::load_random_balls,
    },
};
use clap::ValueEnum;
use glam::Vec3;
use log::warn;

mod cornell_box;
mod cornell_box_glass;
mod mixed_light;
mod random_balls;
#[derive(Default, Debug, Clone, Copy, ValueEnum)]
pub enum SceneType {
    #[default]
    CornellBox,
    CornellBoxGlass,
    RandomBalls,
    MixedLight,
}
#[derive(Clone)]
pub struct Scene {
    pub objects: Vec<Hittable>,
    pub raytrace_config: RaytraceConfig,
    pub camera_config: CameraConfig,
}

pub fn load_scene(scene_type: SceneType) -> Scene {
    let scene = match scene_type {
        SceneType::CornellBox => load_cornell_box(),
        SceneType::CornellBoxGlass => load_cornell_box_glass(),
        SceneType::RandomBalls => load_random_balls(),
        SceneType::MixedLight => load_mixed_light(),
    };
    if scene.raytrace_config.max_depth as u64 * scene.raytrace_config.aa_samples as u64
        > u32::MAX as u64
    {
        warn!(
            "this scene's max_depth*aa_samples is greater than u32::MAX. this can lead to overflows (incorrect stats) when counting total rays in the gpu impl."
        );
    }
    scene
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
/// remember: there's no face behind camera
pub fn make_cornell_box(light_colour: Rgb) -> (Vec<Hittable>, CameraConfig) {
    // 100x100x100 volume
    let left_wall = Parallelogram::new(
        Vec3::new(-50.0, 0.0, 0.0),
        Vec3::new(0.0, 100.0, 0.0),
        Vec3::new(0.0, 0.0, -100.0),
        Diffuse::new(Rgb::new(1.0, 0.0, 0.0)).into(),
    )
    .into();
    let right_wall = Parallelogram::new(
        Vec3::new(50.0, 0.0, 0.0),
        Vec3::new(0.0, 100.0, 0.0),
        Vec3::new(0.0, 0.0, -100.0),
        Diffuse::new(Rgb::new(0.0, 1.0, 0.0)).into(),
    )
    .into();
    let floor = Parallelogram::new(
        Vec3::new(-50.0, 0.0, 0.0),
        Vec3::new(100.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -100.0),
        Diffuse::new(Rgb::WHITE).into(),
    )
    .into();
    let ceiling = Parallelogram::new(
        Vec3::new(-50.0, 100.0, 0.0),
        Vec3::new(100.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -100.0),
        Diffuse::new(Rgb::WHITE).into(),
    )
    .into();
    let back_wall = Parallelogram::new(
        Vec3::new(-50.0, 0.0, -100.0),
        Vec3::new(100.0, 0.0, 0.0),
        Vec3::new(0.0, 100.0, 0.0),
        Diffuse::new(Rgb::WHITE).into(),
    )
    .into();
    let light_source = Parallelogram::new(
        Vec3::new(-25.0, 99.9, -35.0),
        Vec3::new(50.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -30.0),
        DiffuseLight::new(light_colour).into(),
    )
    .into();
    (
        vec![
            left_wall,
            right_wall,
            floor,
            ceiling,
            back_wall,
            light_source,
        ],
        CameraConfig {
            centre: Vec3::new(0.0, 50.0, 137.4), // 50/tan(40/2)
            look_at_centre: Vec3::new(0.0, 50.0, -50.0),
            vertical_fov: 40.0,
            lens_radius: 0.0,
        },
    )
}
