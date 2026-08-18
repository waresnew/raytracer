use glam::Vec3;

use crate::{
    camera::CameraConfig,
    hittable::{Hittable, parallelogram::Parallelogram},
    material::{diffuse::Diffuse, diffuse_light::DiffuseLight},
    renderer::RenderConfig,
    rgb::Rgb,
    scenes::Scene,
};

pub fn load_cornell_box() -> Scene {
    const LIGHT_STRENGTH: f32 = 4.0;
    // 100x100x100 volume
    let left_wall = Box::new(Parallelogram::new(
        Vec3::new(-50.0, 0.0, 0.0),
        Vec3::new(0.0, 100.0, 0.0),
        Vec3::new(0.0, 0.0, -100.0),
        Box::new(Diffuse::new(Rgb::new(1.0, 0.0, 0.0))),
    ));
    let right_wall = Box::new(Parallelogram::new(
        Vec3::new(50.0, 0.0, 0.0),
        Vec3::new(0.0, 100.0, 0.0),
        Vec3::new(0.0, 0.0, -100.0),
        Box::new(Diffuse::new(Rgb::new(0.0, 1.0, 0.0))),
    ));
    let floor = Box::new(Parallelogram::new(
        Vec3::new(-50.0, 0.0, 0.0),
        Vec3::new(100.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -100.0),
        Box::new(Diffuse::new(Rgb::WHITE)),
    ));
    let ceiling = Box::new(Parallelogram::new(
        Vec3::new(-50.0, 100.0, 0.0),
        Vec3::new(100.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -100.0),
        Box::new(Diffuse::new(Rgb::WHITE)),
    ));
    let front_wall = Box::new(Parallelogram::new(
        Vec3::new(-50.0, 0.0, -100.0),
        Vec3::new(100.0, 0.0, 0.0),
        Vec3::new(0.0, 100.0, 0.0),
        Box::new(Diffuse::new(Rgb::WHITE)),
    ));
    let behind_wall = Box::new(Parallelogram::new(
        Vec3::new(-50.0, 0.0, 0.0),
        Vec3::new(100.0, 0.0, 0.0),
        Vec3::new(0.0, 100.0, 0.0),
        Box::new(Diffuse::new(Rgb::WHITE)),
    ));
    let light_source = Box::new(Parallelogram::new(
        Vec3::new(-25.0, 99.9, -35.0),
        Vec3::new(50.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -30.0),
        Box::new(DiffuseLight::new(Rgb::new(
            LIGHT_STRENGTH,
            LIGHT_STRENGTH,
            LIGHT_STRENGTH,
        ))),
    ));
    let objects: Vec<Box<dyn Hittable>> = vec![
        left_wall,
        right_wall,
        floor,
        ceiling,
        front_wall,
        behind_wall,
        light_source,
    ];
    Scene {
        objects,
        render_config: RenderConfig {
            aa_samples: 100,
            max_depth: 50,
            sky_colour: Rgb::BLACK,
        },
        camera_config: CameraConfig {
            centre: Vec3::new(0.0, 50.0, -10.0),
            look_at_centre: Vec3::new(0.0, 50.0, -30.0),
            vertical_fov: 90.0,
            lens_radius: 0.0,
        },
    }
}
