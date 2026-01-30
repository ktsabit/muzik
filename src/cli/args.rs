use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "muzik", about = "Heh")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Scan a directory for music
    Root {
        #[command(subcommand)]
        command: RootCommands,
    },
}

#[derive(Subcommand)]
pub enum RootCommands {
    /// List roots
    List,
    /// Add a root
    Add {
        path: PathBuf,
    },
    /// Remove a root
    Remove {
        id: i64,
    },
}

