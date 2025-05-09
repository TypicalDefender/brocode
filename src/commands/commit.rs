use crate::config::Config;
use crate::git;
use crate::openai;
use crate::utils;
use crate::commands::Command;
use anyhow::{Context, Result, anyhow};
use async_trait::async_trait;
use colored::Colorize;
use dialoguer::{Confirm, Editor};
use std::process::{Command as ProcessCommand};

pub struct CommitCommand {
    config: Config,
}

impl CommitCommand {
    pub fn new(config: Config) -> Self {
        Self { config }
    }
}

#[async_trait]
impl Command for CommitCommand {
    async fn execute(&self, message: Option<String>) -> Result<()> {
        // Check if we're in a git repository
        let repo_path = git::get_git_repo_path()?;
        println!("{} Git repository found at: {}", "✓".green(), repo_path.display());

        // Get the diff of uncommitted changes
        let diff = git::get_uncommitted_diff()?;
        if diff.is_empty() {
            return Err(anyhow!("No changes to commit. Make some changes first!"));
        }

        println!("{} Found uncommitted changes", "✓".green());

        // Get or generate commit message
        let commit_message = match message {
            Some(msg) => msg,
            None => {
                // Check API key
                let api_key = match &self.config.openai.api_key {
                    Some(key) => key.clone(),
                    None => std::env::var("OPENAI_API_KEY")
                        .context("OpenAI API key not found in config or environment. Please set OPENAI_API_KEY environment variable or add it to the config file.")?
                };

                println!("{} Generating commit message using OpenAI...", "⚙️".blue());
                
                // Generate commit message using OpenAI
                let generated_message = openai::generate_commit_message(
                    &api_key,
                    &self.config.openai.model,
                    self.config.openai.temperature,
                    self.config.openai.max_tokens,
                    &self.config.openai.system_prompt,
                    &diff,
                ).await?;
                
                println!("{} Generated commit message:", "✓".green());
                println!("{}", "-".repeat(50));
                println!("{}", generated_message);
                println!("{}", "-".repeat(50));
                
                // Allow user to edit the generated message
                if Confirm::new().with_prompt("Would you like to edit this message?").interact()? {
                    match Editor::new().edit(&generated_message)? {
                        Some(edited) => edited,
                        None => generated_message,
                    }
                } else {
                    generated_message
                }
            }
        };

        // Confirm the commit
        if !Confirm::new()
            .with_prompt("Do you want to commit with this message?")
            .interact()?
        {
            println!("Commit cancelled.");
            return Ok(());
        }

        // Split the commit message into summary and description
        let lines: Vec<&str> = commit_message.split('\n').collect();
        let summary = lines[0];
        let description = if lines.len() > 1 {
            Some(lines[1..].join("\n").trim())
        } else {
            None
        };

        // Perform the git commit
        println!("{} Committing changes...", "⚙️".blue());
        let status = if let Some(desc) = description {
            ProcessCommand::new("git")
                .args(["commit", "-m", summary, "-m", desc])
                .status()?
        } else {
            ProcessCommand::new("git")
                .args(["commit", "-m", summary])
                .status()?
        };

        if status.success() {
            println!("{} Successfully committed changes", "✓".green());
            Ok(())
        } else {
            Err(anyhow!("Failed to commit changes. Git exited with non-zero status."))
        }
    }
}
