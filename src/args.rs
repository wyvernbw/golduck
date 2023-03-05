use clap::Args;
use clap::Parser;
use clap::Subcommand;

#[derive(Parser, Debug)]
#[command(
    author = "wyvernbw",
    about = "🦆🌊 Golduck. a cli program to run and debug godot scenes, built in rust 🦀",
    version = "0.1.2"
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
    /// Run a specific scene, supports fuzzy finding
    Run(Scene),
    /// Debug a specific scene, supports fuzzy finding
    Debug(Scene),
}

#[derive(Args, Debug)]
pub struct Scene {
    pub name: Option<String>,
}
