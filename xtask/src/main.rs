use std::{
    fmt::Debug,
    process::{Command, CommandArgs, ExitStatus},
};

use clap::{builder::OsStr, Parser, Subcommand, ValueEnum};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short, long, value_name = "Project")]
    project: Project,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Clone, Debug)]
enum Commands {
    Build,
}

#[derive(ValueEnum, Clone, Copy)]
enum Project {
    Wasm,
    Cli,
}

fn assert_success(command: &mut Command) -> Result<(), Error> {
    match command.status()?.code() {
        Some(0) => Ok(()),
        e => {
            return Err(CommandExitError {
                args: command
                    .get_args()
                    .map(|i| i.to_string_lossy().into_owned())
                    .collect(),
                command: command.get_program().to_string_lossy().into_owned(),
                exit_code: e,
            })?
        }
    }
}

struct CommandExitError {
    args: Vec<String>,
    command: String,
    exit_code: Option<i32>,
}

impl Debug for CommandExitError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Process {:?} {:?} exited with status code {:?}",
            self.command, self.args, self.exit_code
        )
    }
}

#[derive(Debug)]
enum Error {
    #[allow(unused)]
    Io(std::io::Error),
    #[allow(unused)]
    Command(CommandExitError),
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Error::Io(value)
    }
}

impl From<CommandExitError> for Error {
    fn from(value: CommandExitError) -> Self {
        Error::Command(value)
    }
}

fn main() -> Result<(), Error> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Build => match cli.project {
            Project::Wasm => {
                assert_success(Command::new("cargo").args([
                    "build",
                    "--package",
                    "painrose-wasm",
                    "--target",
                    "wasm32-unknown-unknown",
                ]))?;

                assert_success(Command::new("wasm-bindgen").args([
                    "target/wasm32-unknown-unknown/debug/painrose_wasm.wasm",
                    "--out-dir",
                    "js-build",
                    "--target",
                    "web",
                ]))?;

                std::fs::copy("painrose-wasm/index.html", "js-build/index.html")?;

                assert_success(Command::new("python3").args([
                    "-m",
                    "http.server",
                ]).current_dir("js-build"))?;
            }
            _ => todo!(),
        },
    }

    return Ok(());
}
