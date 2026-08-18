use clap::Parser;
use glam::Vec2;
use image::DynamicImage;
use indicatif::{HumanDuration, MultiProgress, ProgressBar, ProgressStyle};
use indicatif_log_bridge::LogWrapper;
use log::info;
use raytracer::{
    bvh::BvhNode,
    camera::Camera,
    renderer::Renderer,
    scenes::{self},
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

    let scene = scenes::load_scene(cli.scene);
    let camera = Camera::new(Vec2::new(width as f32, height as f32), scene.camera_config);
    let renderer = Renderer::new(height, width, camera, scene.render_config);
    let bvh = BvhNode::from_objects(scene.objects);
    let img = renderer.render(&bvh, &progress_bar);

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
