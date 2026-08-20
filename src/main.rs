use clap::Parser;
use image::{DynamicImage, RgbImage};
use indicatif::{HumanDuration, MultiProgress};
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
    let multi = MultiProgress::new();

    let raytracer = RaytracerFacade::new(cli.cpu, scene, cli.gpu_chunk_height);
    let progress_bar = raytracer.progress_bar().clone();
    multi.add(progress_bar.clone());
    setup_logging(&cli, &multi);
    let (img, stats) = raytracer.render();
    progress_bar.finish_and_clear();

    save_output(cli.output, img);
    info!(
        "Done in {}, total rays: {}",
        HumanDuration(progress_bar.elapsed()),
        stats.total_rays
    );
}

fn setup_logging(cli: &Cli, multi: &MultiProgress) {
    let logger = env_logger::Builder::new()
        .filter_level(cli.verbosity.log_level_filter())
        .build();
    let level = logger.filter();
    LogWrapper::new(multi.clone(), logger).try_init().unwrap();
    log::set_max_level(level); //workaround from indicatif-log-bridge
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
