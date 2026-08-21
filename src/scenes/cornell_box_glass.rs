use glam::Vec3;

use crate::{
    hittable::sphere::Sphere,
    material::{diffuse::Diffuse, glass::Glass},
    raytracer::RaytraceConfig,
    rgb::Rgb,
    scenes::{Scene, make_box, make_cornell_box},
};

pub fn load_cornell_box_glass() -> Scene {
    const LIGHT_STRENGTH: f32 = 4.0;
    const LIGHT_COLOUR: Rgb = Rgb::new(1.0, 0.84, 0.67);
    let (mut cornell_box, camera_config) = make_cornell_box(LIGHT_COLOUR * LIGHT_STRENGTH);
    let tall_box = make_box(
        Diffuse::new(Rgb::WHITE).into(),
        Vec3::new(-15.0, 30.0, -60.0),
        20.0,
        Vec3::new(30.0, 60.0, 30.0),
    );
    let sphere = Sphere::new(Vec3::new(15.0, 15.0, -30.0), 15.0, Glass::new(1.5).into()).into();

    cornell_box.extend(tall_box);
    cornell_box.push(sphere);
    Scene {
        objects: cornell_box,
        raytrace_config: RaytraceConfig {
            aa_samples: 10_000,
            max_depth: 8,
            sky_colour: Rgb::BLACK,
            image_width: 800,
            image_height: 800,
        },
        camera_config,
    }
}
