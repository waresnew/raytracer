use clap::{ArgAction, Parser};
use clap_verbosity_flag::{Verbosity, WarnLevel};
use display_info::DisplayInfo;
use log::warn;
use raytracer::scenes::SceneType;
use viuer::KittySupport;

#[derive(Debug, Clone, Parser)]
#[command(version, about, long_about=None, disable_help_flag=true)]
pub struct Cli {
    /// Height of the output image in pixels. Defaults to half of the screen height.
    #[arg(short, long)]
    pub height: Option<u32>,

    /// Width of the output image in pixels. Defaults to half of the screen width.
    #[arg(short, long)]
    pub width: Option<u32>,

    /// Specify an output file name. If this is not provided then the output image will be printed to the terminal.
    #[arg(short, long)]
    pub output: Option<String>,

    /// Which scene to render.
    #[arg(short,long,value_enum, default_value_t=SceneType::CornellBox)]
    pub scene: SceneType,

    #[command(flatten)]
    pub verbosity: Verbosity<WarnLevel>,

    /// Prints this help message.
    #[arg(short='?', long, action=ArgAction::Help)]
    help: (),
}
impl Cli {
    pub fn compute_defaults(mut self) -> Self {
        if self.output.is_none()
            && !viuer::is_iterm_supported()
            && viuer::get_kitty_support() == KittySupport::None
        {
            warn!(
                "No output file was selected but the environment doesn't have iTerm nor Kitty graphics protocol support. Either switch to file-based output or use a supported terminal"
            )
        }
        let screen_info = DisplayInfo::all()
            .unwrap()
            .into_iter()
            .find(|x| x.is_primary)
            .expect("no primary monitor found");
        if self.height.is_none() {
            self.height = Some(screen_info.height / 2);
        }
        if self.width.is_none() {
            self.width = Some(screen_info.width / 2);
        }
        self
    }
}
