use glam::Vec3;
use rand::{RngExt, SeedableRng, rngs::SmallRng};

use crate::{
    camera::CameraConfig,
    hittable::{Hittable, parallelogram::Parallelogram, sphere::Sphere},
    material::{Material, diffuse::Diffuse, glass::Glass, metal::Metal},
    renderer::RenderConfig,
    rgb::Rgb,
    scenes::Scene,
};

pub fn load_random_balls() -> Scene {
    let mut rng = SmallRng::seed_from_u64(1234);
    let mut objects: Vec<Hittable> = Vec::new();
    let floor = Hittable::Parallelogram(Parallelogram::new(
        Vec3::new(-100.0, 0.0, 100.0),
        Vec3::new(200.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -200.0),
        Material::Diffuse(Diffuse::new(Rgb::new(0.5, 0.5, 0.5))),
    ));
    objects.push(floor);
    for x in -10..10 {
        for z in -10..10 {
            let material_rand = rng.random_range(0.0..1.0);
            let material: Material = if material_rand < 0.1 {
                Material::Glass(Glass::new(1.5))
            } else if material_rand < 0.4 {
                Material::Metal(Metal::new(Rgb::random(), rng.random_range(0.0..1.0)))
            } else {
                Material::Diffuse(Diffuse::new(Rgb::random()))
            };
            const SPACING: f32 = 0.8;
            let pos = Vec3::new(
                x as f32 * SPACING + rng.random_range(-0.25..0.25),
                rng.random_range(0.25..0.5),
                z as f32 * SPACING + rng.random_range(-0.25..0.25),
            );
            objects.push(Hittable::Sphere(Sphere::new(pos, 0.25, material)))
        }
    }
    objects.push(Hittable::Sphere(Sphere::new(
        Vec3::new(0.0, 1.0, 0.0),
        1.0,
        Material::Glass(Glass::new(1.5)),
    )));
    objects.push(Hittable::Sphere(Sphere::new(
        Vec3::new(4.0, 1.0, 0.0),
        1.0,
        Material::Metal(Metal::new(Rgb::new(0.5, 0.5, 0.5), 0.0)),
    )));
    objects.push(Hittable::Sphere(Sphere::new(
        Vec3::new(-4.0, 1.0, 0.0),
        1.0,
        Material::Diffuse(Diffuse::new(Rgb::new(1.0, 0.3, 0.5))),
    )));
    Scene {
        objects,
        render_config: RenderConfig {
            aa_samples: 100,
            max_depth: 50,
            sky_colour: Rgb::new(0.53, 0.81, 0.92),
        },
        camera_config: CameraConfig {
            centre: Vec3::new(13.0, 2.0, 3.0),
            vertical_fov: 20.0,
            lens_radius: 0.02,
            look_at_centre: Vec3::new(0.0, 0.0, -1.0),
        },
    }
}
