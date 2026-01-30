use crate::engine::MuzikEngine;

use super::args::{Cli, Commands, RootCommands};

pub async fn handle(cli: Cli, engine: MuzikEngine) -> anyhow::Result<()> {
    match cli.command {
        Commands::Root { command } => handle_root(command, engine).await?,
    }
    Ok(())
}

pub async fn handle_root(command: RootCommands, engine: MuzikEngine) -> anyhow::Result<()>{
    match command {
        RootCommands::Add { path } => {
            let id = engine.add_root(path).await?;
            println!("Added root: {}", id)
        }
        RootCommands::List => {
            let roots = engine.list_roots().await?;
            for root in roots {
                println!("Root [id:{:?}]: {:?}", root.id,root.path);
            }
        },
        RootCommands::Remove { id } => {
            engine.remove_root(id).await?;
            println!("Removed root: {}", id)
        }
    }
    Ok(())
}
