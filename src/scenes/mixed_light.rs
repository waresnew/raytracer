use glam::Vec3;

use crate::{
    camera::CameraConfig,
    hittable::{Hittable, parallelogram::Parallelogram, sphere::Sphere},
    material::{diffuse::Diffuse, diffuse_light::DiffuseLight},
    renderer::RenderConfig,
    rgb::Rgb,
    scenes::Scene,
};

/// to show rgb mixing
pub fn load_mixed_light() -> Scene {
    let floor = Box::new(Parallelogram::new(
        Vec3::new(-100.0, 0.0, 100.0),
        Vec3::new(200.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -200.0),
        Box::new(Diffuse::new(Rgb::new(0.5, 0.5, 0.5))),
    ));
    const LIGHT_STRENGTH: f32 = 10.0;
    let red = Box::new(Sphere::new(
        Vec3::new(-1.2, 1.0, 1.0),
        1.0,
        Box::new(DiffuseLight::new(Rgb::new(LIGHT_STRENGTH, 0.0, 0.0))),
    ));
    let green = Box::new(Sphere::new(
        Vec3::new(1.2, 1.0, 1.0),
        1.0,
        Box::new(DiffuseLight::new(Rgb::new(0.0, LIGHT_STRENGTH, 0.0))),
    ));
    let blue = Box::new(Sphere::new(
        Vec3::new(0.0, 1.0, -1.2),
        1.0,
        Box::new(DiffuseLight::new(Rgb::new(0.0, 0.0, LIGHT_STRENGTH))),
    ));
    let objects: Vec<Box<dyn Hittable>> = vec![floor, red, green, blue];
    Scene {
        objects,
        render_config: RenderConfig {
            aa_samples: 100,
            max_depth: 50,
            sky_colour: Rgb::BLACK,
        },
        camera_config: CameraConfig {
            centre: Vec3::new(0.0, 5.0, 0.1),
            look_at_centre: Vec3::new(0.0, 0.0, 0.0),
            vertical_fov: 90.0,
            lens_radius: 0.0,
        },
    }
}
