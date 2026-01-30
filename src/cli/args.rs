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
    /// Manage root directories
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
    /// Scan all roots
    Scan {
        root_id: Option<i64>,
    }
}

