use clap::Parser;
use glam::{Vec2, Vec3};
use image::DynamicImage;
use raytracer::{camera::Camera, renderer::Renderer, viewport::Viewport};
use viuer::Config;

use crate::cli::Cli;

mod cli;
fn main() {
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
    let renderer = Renderer {
        camera,
        height,
        width,
    };
    let img = renderer.render_to_image();
    if let Some(output_file) = cli.output {
        img.save(output_file).unwrap();
    } else {
        viuer::print(
            &DynamicImage::ImageRgb8(img),
            &Config {
                absolute_offset: false,
                height: Some((viuer::terminal_size().1 - 4) as u32),
                ..Default::default()
            },
        )
        .unwrap();
    }
}
