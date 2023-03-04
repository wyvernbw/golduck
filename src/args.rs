use clap::Args;
use clap::Parser;
use clap::Subcommand;

#[derive(Parser, Debug)]
#[command(author, about, version)]
pub struct GolduckArgs {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Run(Scene),
    Debug(Scene),
}

impl Commands {
    pub fn scene_name(&self) -> &Option<String> {
        match self {
            Commands::Run(scene) => &scene.name,
            Commands::Debug(scene) => &scene.name,
        }
    }
}

#[derive(Args, Debug)]
pub struct Scene {
    pub name: Option<String>,
}
