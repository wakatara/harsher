use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "harsh")]
#[command(
    author,
    version,
    about = "Minimalist CLI habit tracker",
    long_about = "A minimalist CLI for tracking, understanding, and forging habits."
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    /// Custom config directory if not the OS default
    #[arg(long = "config", short = 'c', value_name = "DIR", global = true)]
    config: Option<PathBuf>,

    /// Enable colored output (auto/always/never)
    #[arg(long = "color", short = 'C', default_value = "auto", global = true)]
    color: ColorOption,
}

#[derive(clap::ValueEnum, Clone, Debug)]
enum ColorOption {
    Auto,
    Always,
    Never,
}

#[derive(Subcommand)]
enum Commands {
    /// Show consistency graph
    #[command(alias = "l")]
    Log {},
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    // Run command (default is Ask)
    match cli.command {
        Some(Commands::Log {}) => {
            /// commands::log::run();
            commands::log::run();
        }
        None => {
            // Default: run log
            // commands::log::run();
            commands::log::run();
        }
    }

    Ok(())
}

mod commands {
    pub mod ask;
    pub mod log;
    pub mod stats;
    pub mod todo;
}

pub mod models;
