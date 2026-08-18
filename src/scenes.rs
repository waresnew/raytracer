use crate::{
    camera::CameraConfig,
    hittable::Hittable,
    renderer::RenderConfig,
    scenes::{
        cornell_box::load_cornell_box, mixed_light::load_mixed_light,
        random_balls::load_random_balls,
    },
};
use clap::ValueEnum;

mod cornell_box;
mod mixed_light;
mod random_balls;
#[derive(Default, Debug, Clone, Copy, ValueEnum)]
pub enum SceneType {
    #[default]
    CornellBox,
    RandomBalls,
    MixedLight,
}
pub struct Scene {
    pub objects: Vec<Box<dyn Hittable>>,
    pub render_config: RenderConfig,
    pub camera_config: CameraConfig,
}

pub fn load_scene(scene: SceneType) -> Scene {
    match scene {
        SceneType::CornellBox => load_cornell_box(),
        SceneType::RandomBalls => load_random_balls(),
        SceneType::MixedLight => load_mixed_light(),
    }
}
