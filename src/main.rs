mod cli;
mod commands;
mod config;
mod git;
mod openai;
mod utils;

use anyhow::Result;
use cli::Cli;
use clap::Parser;
use config::Config;
use commands::Command;
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    
    // Parse command line arguments
    let cli = Cli::parse();

    // Load configuration
    let config_path = config::get_config_path()?;
    let config = Config::load(&config_path).unwrap_or_else(|_| {
        println!("No configuration found. Creating default configuration at {:?}", config_path);
        let config = Config::default();
        if let Err(e) = config.save(&config_path) {
            eprintln!("Failed to save default configuration: {}", e);
        }
        config
    });

    // Execute the requested command
    match cli.command {
        cli::Commands::Commit { message } => {
            let cmd = commands::commit::CommitCommand::new(config);
            cmd.execute(message).await?;
        }
    }

    Ok(())
}
