use std::time::Instant;

use clap::Parser;
use glam::{Vec2, Vec3};
use image::DynamicImage;
use log::info;
use raytracer::{
    camera::Camera, hittable::sphere::Sphere, renderer::Renderer, viewport::Viewport, world::World,
};
use viuer::Config;

use crate::cli::Cli;

mod cli;
fn main() {
    let start = Instant::now();
    let cli = Cli::parse().compute_defaults();
    env_logger::Builder::new()
        .filter_level(cli.verbosity.log_level_filter())
        .init();
    let height = cli.height.unwrap();
    let width = cli.width.unwrap();
    const VIEWPORT_HEIGHT: f32 = 2.0;
    let aspect_ratio = width as f32 / height as f32;
    let viewport = Viewport::from_centre(
        Vec2::new(0.0, 0.0),
        Vec2::new(VIEWPORT_HEIGHT * aspect_ratio, VIEWPORT_HEIGHT),
    );
    let camera = Camera {
        centre: Vec3::ZERO,
        viewport,
        focal_length: 1.0,
    };
    let renderer = Renderer::new(camera, height, width);
    let world = World::from_objects(vec![
        Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)),
        Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)),
    ]);
    let img = renderer.render_world(&world);
    if let Some(output_file) = cli.output {
        img.save(output_file).unwrap();
    } else {
        viuer::print(
            &DynamicImage::ImageRgb8(img),
            &Config {
                absolute_offset: false,
                ..Default::default()
            },
        )
        .unwrap();
    }
    info!("Took {:.1}s", start.elapsed().as_millis() as f32 / 1000.0);
}
