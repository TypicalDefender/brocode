use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Generate a commit message and commit changes
    Commit {
        /// Optional manual commit message (skips AI generation)
        #[arg(short, long)]
        message: Option<String>,
    },
}
