use clap::Parser;
use glam::{Vec2, Vec3};
use image::DynamicImage;
use indicatif::{HumanDuration, MultiProgress, ProgressBar, ProgressStyle};
use indicatif_log_bridge::LogWrapper;
use log::info;
use raytracer::{
    bvh::BvhNode,
    camera::Camera,
    hittable::{parallelogram::Parallelogram, sphere::Sphere},
    material::{diffuse::Diffuse, diffuse_light::DiffuseLight, glass::Glass, metal::Metal},
    renderer::{RenderConfig, Renderer},
    rgb::Rgb,
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

    const VERTICAL_FOV: f32 = 90.0;
    const LENS_RADIUS: f32 = 0.01;
    let camera = Camera::new(
        Vec3::new(0.0, 0.75, 1.0),
        Vec3::new(0.0, 0.0, -1.0),
        Vec2::new(width as f32, height as f32),
        VERTICAL_FOV,
        LENS_RADIUS,
    );
    let renderer = Renderer::new(camera, height, width);
    let bvh = BvhNode::from_objects(vec![
        Box::new(Parallelogram::new(
            Vec3::new(-100.0, 0.0, 100.0),
            Vec3::new(200.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, -200.0),
            Diffuse::new(Rgb::new(0.5, 0.5, 0.5)),
        )),
        Box::new(Sphere::new(
            Vec3::new(0.75, 0.5, -1.0),
            0.5,
            Glass::new(1.5),
        )),
        Box::new(Sphere::new(
            Vec3::new(-0.75, 0.5, -1.0),
            0.5,
            Metal::new(Rgb::new(0.7, 0.5, 0.0), 0.1),
        )),
        Box::new(Sphere::new(
            Vec3::new(0.75, 1.0, -3.0),
            1.0,
            Metal::new(Rgb::new(0.2, 1.0, 1.0), 0.3),
        )),
        Box::new(Parallelogram::new(
            Vec3::new(-10.0, 0.0, -10.0),
            Vec3::new(0.0, 10.0, 0.0),
            Vec3::new(20.0, 0.0, 0.0),
            DiffuseLight::new(Rgb::new(5.0, 0.5, 0.5)),
        )),
    ]);
    let img = renderer.render(
        &bvh,
        &progress_bar,
        RenderConfig {
            aa_samples: cli.aa_samples,
            max_depth: cli.max_depth,
        },
    );

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
