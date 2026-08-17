use clap::Parser;
use glam::{Vec2, Vec3};
use image::DynamicImage;
use indicatif::{HumanDuration, MultiProgress, ProgressBar, ProgressStyle};
use indicatif_log_bridge::LogWrapper;
use log::info;
use raytracer::{
    camera::Camera,
    hittable::sphere::Sphere,
    material::{Material, diffuse::Diffuse, glass::Glass, metal::Metal},
    renderer::Renderer,
    rgb::Rgb,
    viewport::Viewport,
    world::World,
};
use viuer::Config;

use crate::cli::Cli;

mod cli;
fn setup_logging(cli: &Cli, multi: &MultiProgress) {
    let logger = env_logger::Builder::new()
        .filter_level(cli.verbosity.log_level_filter())
        .build();
    let level = logger.filter();
    LogWrapper::new(multi.clone(), logger).try_init().unwrap();
    log::set_max_level(level); //workaround from indicatif-log-bridge
}
fn main() {
    let cli = Cli::parse().compute_defaults();
    let height = cli.height.unwrap();
    let width = cli.width.unwrap();
    let multi = MultiProgress::new();
    setup_logging(&cli, &multi);
    let progress_bar = ProgressBar::new(height as u64).with_style(
        ProgressStyle::with_template(
            "\t[{elapsed_precise}] {wide_bar:.green/red} {pos:>7}/{len:7} ETA: {eta}\t",
        )
        .unwrap(),
    );
    multi.add(progress_bar.clone());

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
        Box::new(Sphere::new(
            Vec3::new(0.0, -100.5, -1.0),
            100.0,
            Diffuse::new(Rgb::new(0.5, 0.5, 0.5)),
        )),
        Box::new(Sphere::new(
            Vec3::new(0.75, 0.0, -1.0),
            0.5,
            Glass::new(1.5),
        )),
        Box::new(Sphere::new(
            Vec3::new(-0.75, 0.0, -1.0),
            0.5,
            Metal::new(Rgb::new(0.7, 0.5, 0.0), 0.1),
        )),
        Box::new(Sphere::new(
            Vec3::new(0.75, 0.0, -2.0),
            0.5,
            Metal::new(Rgb::new(0.2, 0.2, 1.0), 0.3),
        )),
    ]);
    let img = renderer.render_world(&world, &progress_bar);

    progress_bar.finish_and_clear();
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
    info!("Done in {}", HumanDuration(progress_bar.elapsed()));
}
