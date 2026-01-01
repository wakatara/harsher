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
    /// Ask for habit status interactively
    #[command(alias = "a")]
    Ask {
        /// Filter by habit name (substring), date (YYYY-MM-DD), or 'yday' for yesterday
        filter: Option<String>,
    },

    /// Show consistency graph
    #[command(alias = "l")]
    Log {
        #[command(subcommand)]
        subcommand: Option<LogSubcommand>,

        /// Filter habits by name (substring)
        #[arg(trailing_var_arg = true)]
        filter: Option<String>,
    },

    /// Show undone habits for today
    #[command(alias = "t")]
    Todo,

    /// Display version information
    #[command(alias = "v")]
    Version,
}

#[derive(Subcommand)]
enum LogSubcommand {
    /// Show long-term statistics for all habits
    #[command(alias = "s")]
    Stats,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    // Run command (default is Ask)
    match cli.command {
        Some(Commands::Ask { filter }) => {
            /// commands::ask::run(&mut repo, filter, use_color)?;
            commands::ask::run();
        }
        Some(Commands::Log { subcommand, filter }) => match subcommand {
            Some(LogSubcommand::Stats) => {
                // commands::stats::run(&repo, use_color)?;
                commands::stats::run();
            }
            None => {
                // commands::log::run(&repo, filters, use_color)?;
                commands::log::run();
            }
        },
        Some(Commands::Todo) => {
            // commands::todo::run(&repo, use_color)?;
            commands::todo::run();
        }
        Some(Commands::Version) => {
            // Already handled above
            unreachable!()
        }
        None => {
            // Default: run ask
            // commands::ask::run(&mut repo, None, use_color)?;
            commands::ask::run();
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
