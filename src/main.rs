mod args;
use args::GolduckArgs;
use clap::Parser;
use inquire::Select;
use std::{fs::DirEntry, io, path::Path};

#[derive(Debug)]
enum UserError {
    NoScenesFound,
    PromptError,
}

impl std::fmt::Display for UserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use UserError::*;
        match self {
            NoScenesFound => write!(f, "no scenes found in the current workspace"),
            PromptError => write!(f, "error while prompting for scene name"),
        }
    }
}

impl std::error::Error for UserError {}

fn walk_dir(dir: &Path) -> io::Result<Box<dyn Iterator<Item = DirEntry>>> {
    let dir = std::fs::read_dir(dir)?;
    let dir = Box::new(dir.flatten()) as Box<dyn Iterator<Item = DirEntry>>;
    let dir = dir
        .flat_map(|entry| {
            let path = entry.path();
            if path.is_dir() {
                walk_dir(&path).ok()
            } else {
                Some(Box::new(std::iter::once(entry)) as Box<dyn Iterator<Item = DirEntry>>)
            }
        })
        .flatten();
    Ok(Box::new(dir))
}

fn get_scenes() -> Result<Vec<String>, UserError> {
    let entries = walk_dir(Path::new("."))
        .map_err(|_| UserError::NoScenesFound)?
        .filter(|entry| {
            let path = entry.path();
            let ext = path.extension().and_then(|ext| ext.to_str());
            let ext = match ext {
                Some(ext) => ext,
                None => return false,
            };
            matches!(ext, "tscn" | "scn")
        })
        .map(|entry| entry.path().to_string_lossy().to_string())
        .collect::<Vec<_>>();
    if entries.is_empty() {
        return Err(UserError::NoScenesFound);
    }
    Ok(entries)
}

fn ask_for_scene_name() -> anyhow::Result<String> {
    let scenes = get_scenes()?;
    let answer = Select::new("Select a scene to run:", scenes)
        .prompt()
        .map_err(|_| UserError::PromptError)?;
    Ok(answer)
}

fn godot_process(args: &args::GolduckArgs) -> std::process::Command {
    let mut process = std::process::Command::new("godot");
    process
        .stdout(match args.silence {
            false => std::process::Stdio::inherit(),
            true => std::process::Stdio::null(),
        })
        .stderr(match args.no_error {
            false => std::process::Stdio::inherit(),
            true => std::process::Stdio::null(),
        });
    process
}

fn main() -> anyhow::Result<()> {
    let args = GolduckArgs::parse();

    use args::Commands::*;
    match args.command {
        Run(_) | Debug(_) => {
            let scene_name = args.command.scene_name();
            let scene_name = match scene_name {
                Some(name) => name.to_owned(),
                None => ask_for_scene_name()?,
            };
            let run_mode = match args.command {
                PlayDebug => unreachable!(),
                Play => unreachable!(),
                Run(_) => "".to_string(),
                Debug(_) => "--debug".to_string(),
            };
            let mut handle = godot_process(&args).args(&[run_mode, scene_name]).spawn()?;
            handle.wait()?;
        }
        Play => {
            let mut handle = godot_process(&args).spawn()?;
            handle.wait()?;
        }
        PlayDebug => {
            let mut handle = godot_process(&args).arg("--debug").spawn()?;
            handle.wait()?;
        }
    }

    Ok(())
}