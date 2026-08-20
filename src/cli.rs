use clap::{ArgAction, Parser};
use clap_verbosity_flag::{Verbosity, WarnLevel};
use log::warn;
use raytracer::scenes::SceneType;
use viuer::KittySupport;

#[derive(Debug, Clone, Parser)]
#[command(version, about, long_about=None, disable_help_flag=true)]
pub struct Cli {
    /// Specify an output file name. If this is not provided then the output image will be printed to the terminal.
    #[arg(short, long)]
    pub output: Option<String>,

    /// Use the CPU for raytracing instead of the GPU.
    #[arg(short, long, default_value_t = false)]
    pub cpu: bool,

    /// Which scene to render.
    #[arg(short,long,value_enum, default_value_t=SceneType::CornellBox)]
    pub scene: SceneType,

    /// When GPU mode is used, each dispatch will process image_width*(this parameter) pixels.
    /// Reduce this to avoid your computer freezing at the cost of slower runtimes. If this parameter is not set, then the program
    /// will process as many pixels as a storage buffer can hold for each dispatch.
    #[arg(long)]
    pub gpu_chunk_height: Option<u32>,

    #[command(flatten)]
    pub verbosity: Verbosity<WarnLevel>,

    /// Prints this help message.
    #[arg(short, long, action=ArgAction::Help)]
    help: (),
}
impl Cli {
    pub fn post_process(self) -> Self {
        if self.output.is_none()
            && !viuer::is_iterm_supported()
            && viuer::get_kitty_support() == KittySupport::None
        {
            warn!(
                "No output file was selected but the environment doesn't have iTerm nor Kitty graphics protocol support. Either switch to file-based output or use a supported terminal"
            )
        }
        self
    }
}
