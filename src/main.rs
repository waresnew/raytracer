use clap::Parser;
use image::{DynamicImage, RgbImage};
use indicatif::{HumanDuration, MultiProgress, ProgressBar, ProgressStyle};
use indicatif_log_bridge::LogWrapper;
use log::info;
use raytracer::{
    raytracer::RaytracerFacade,
    scenes::{self},
};
use viuer::Config;

use crate::cli::Cli;

mod cli;
fn main() {
    let cli = Cli::parse().post_process();
    let scene = scenes::load_scene(cli.scene);
    let (multi, progress_bar) = setup_progress_bar(scene.raytrace_config.image_height as u64);
    setup_logging(&cli, &multi);

    let raytracer = RaytracerFacade::new(cli.cpu, scene);
    let img = raytracer.render(&progress_bar);
    progress_bar.finish_and_clear();

    save_output(cli.output, img);
    info!("Done in {}", HumanDuration(progress_bar.elapsed()));
}

fn setup_logging(cli: &Cli, multi: &MultiProgress) {
    let logger = env_logger::Builder::new()
        .filter_level(cli.verbosity.log_level_filter())
        .build();
    let level = logger.filter();
    LogWrapper::new(multi.clone(), logger).try_init().unwrap();
    log::set_max_level(level); //workaround from indicatif-log-bridge
}
fn setup_progress_bar(bar_len: u64) -> (MultiProgress, ProgressBar) {
    let multi = MultiProgress::new();
    let progress_bar = ProgressBar::new(bar_len).with_style(
        ProgressStyle::with_template(
            "\t[{elapsed_precise}] {wide_bar:.green/red} {pos:>7}/{len:7} ETA: {eta}\t",
        )
        .unwrap(),
    );
    multi.add(progress_bar.clone());
    (multi, progress_bar)
}
fn save_output(output_file: Option<String>, img: RgbImage) {
    if let Some(output_file) = output_file {
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
}
