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
    let floor = Parallelogram::new(
        Vec3::new(-100.0, 0.0, 100.0),
        Vec3::new(200.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -200.0),
        Diffuse::new(Rgb::new(0.5, 0.5, 0.5)).into(),
    )
    .into();
    const LIGHT_STRENGTH: f32 = 10.0;
    let red = Sphere::new(
        Vec3::new(-1.2, 1.0, 1.0),
        1.0,
        DiffuseLight::new(Rgb::new(LIGHT_STRENGTH, 0.0, 0.0)).into(),
    )
    .into();
    let green = Sphere::new(
        Vec3::new(1.2, 1.0, 1.0),
        1.0,
        DiffuseLight::new(Rgb::new(0.0, LIGHT_STRENGTH, 0.0)).into(),
    )
    .into();
    let blue = Sphere::new(
        Vec3::new(0.0, 1.0, -1.2),
        1.0,
        DiffuseLight::new(Rgb::new(0.0, 0.0, LIGHT_STRENGTH)).into(),
    )
    .into();
    let objects: Vec<Hittable> = vec![floor, red, green, blue];
    Scene {
        objects,
        render_config: RenderConfig {
            aa_samples: 100,
            max_depth: 50,
            sky_colour: Rgb::BLACK,
            image_width: 800,
            image_height: 450,
        },
        camera_config: CameraConfig {
            centre: Vec3::new(0.0, 5.0, 0.1),
            look_at_centre: Vec3::new(0.0, 0.0, 0.0),
            vertical_fov: 90.0,
            lens_radius: 0.0,
        },
    }
}
