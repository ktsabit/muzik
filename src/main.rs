
mod db;
mod domain;
mod cli;
mod engine;
use clap::Parser;

#[tokio::main]
async fn main()-> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    let pool = db::init_db().await?;
    let engine = engine::MuzikEngine::new(pool);

    let args = cli::Cli::parse();
    cli::handle(args, engine).await?;


    Ok(())
}
