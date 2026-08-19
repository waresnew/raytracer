use glam::Vec3;

use crate::{
    camera::CameraConfig,
    hittable::{Hittable, parallelogram::Parallelogram},
    material::{Material, diffuse::Diffuse, diffuse_light::DiffuseLight},
    renderer::RenderConfig,
    rgb::Rgb,
    scenes::{Scene, make_box},
};

pub fn load_cornell_box() -> Scene {
    const LIGHT_STRENGTH: f32 = 4.0;
    // 100x100x100 volume
    let left_wall = Hittable::Parallelogram(Parallelogram::new(
        Vec3::new(-50.0, 0.0, 0.0),
        Vec3::new(0.0, 100.0, 0.0),
        Vec3::new(0.0, 0.0, -100.0),
        Material::Diffuse(Diffuse::new(Rgb::new(1.0, 0.0, 0.0))),
    ));
    let right_wall = Hittable::Parallelogram(Parallelogram::new(
        Vec3::new(50.0, 0.0, 0.0),
        Vec3::new(0.0, 100.0, 0.0),
        Vec3::new(0.0, 0.0, -100.0),
        Material::Diffuse(Diffuse::new(Rgb::new(0.0, 1.0, 0.0))),
    ));
    let floor = Hittable::Parallelogram(Parallelogram::new(
        Vec3::new(-50.0, 0.0, 0.0),
        Vec3::new(100.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -100.0),
        Material::Diffuse(Diffuse::new(Rgb::WHITE)),
    ));
    let ceiling = Hittable::Parallelogram(Parallelogram::new(
        Vec3::new(-50.0, 100.0, 0.0),
        Vec3::new(100.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -100.0),
        Material::Diffuse(Diffuse::new(Rgb::WHITE)),
    ));
    let back_wall = Hittable::Parallelogram(Parallelogram::new(
        Vec3::new(-50.0, 0.0, -100.0),
        Vec3::new(100.0, 0.0, 0.0),
        Vec3::new(0.0, 100.0, 0.0),
        Material::Diffuse(Diffuse::new(Rgb::WHITE)),
    ));
    let light_source = Hittable::Parallelogram(Parallelogram::new(
        Vec3::new(-25.0, 99.9, -35.0),
        Vec3::new(50.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -30.0),
        Material::DiffuseLight(DiffuseLight::new(Rgb::new(
            LIGHT_STRENGTH,
            LIGHT_STRENGTH,
            LIGHT_STRENGTH,
        ))),
    ));

    let tall_box = make_box(
        Material::Diffuse(Diffuse::new(Rgb::WHITE)),
        Vec3::new(-15.0, 30.0, -60.0),
        20.0,
        Vec3::new(30.0, 60.0, 30.0),
    );
    let cube_box = make_box(
        Material::Diffuse(Diffuse::new(Rgb::WHITE)),
        Vec3::new(15.0, 15.0, -30.0),
        -20.0,
        Vec3::new(30.0, 30.0, 30.0),
    );

    let mut objects: Vec<Hittable> = vec![
        left_wall,
        right_wall,
        floor,
        ceiling,
        back_wall,
        light_source,
    ];
    objects.extend(tall_box);
    objects.extend(cube_box);
    Scene {
        objects,
        render_config: RenderConfig {
            aa_samples: 100,
            max_depth: 50,
            sky_colour: Rgb::BLACK,
        },
        camera_config: CameraConfig {
            centre: Vec3::new(0.0, 50.0, 137.4), // 50/tan(40/2)
            look_at_centre: Vec3::new(0.0, 50.0, -50.0),
            vertical_fov: 40.0,
            lens_radius: 0.0,
        },
    }
}
