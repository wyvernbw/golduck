use clap::Args;
use clap::Parser;
use clap::Subcommand;

#[derive(Parser, Debug)]
#[command(
    author = "wyvernbw",
    about = "a cli program to run and debug godot scenes, built in rust 🦀",
    version = "0.1.0"
)]
pub struct GolduckArgs {
    #[arg(
        short = 's',
        long = "silence",
        default_value = "false",
        help = "Silence the godot console output"
    )]
    pub silence: bool,
    #[arg(
        short = 'e',
        long = "no-error",
        default_value = "false",
        help = "Silence the godot console errors"
    )]
    pub no_error: bool,
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Play the project (equivalent to pressing F5 in the editor)
    Play,
    /// Play the project in debug mode
    PlayDebug,
    /// Run a specific scene, supports fuzzy findig
    Run(Scene),
    /// Debug a specific scene, supports fuzzy findig
    Debug(Scene),
}

impl Commands {
    pub fn scene_name(&self) -> &Option<String> {
        match self {
            Commands::PlayDebug => &None,
            Commands::Play => &None,
            Commands::Run(scene) => &scene.name,
            Commands::Debug(scene) => &scene.name,
        }
    }
}

#[derive(Args, Debug)]
pub struct Scene {
    pub name: Option<String>,
}
