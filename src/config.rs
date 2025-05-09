use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub openai: OpenAIConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OpenAIConfig {
    pub api_key: Option<String>,
    pub model: String,
    pub temperature: f32,
    pub max_tokens: u32,
    pub system_prompt: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            openai: OpenAIConfig {
                api_key: None,
                model: "gpt-3.5-turbo".to_string(),
                temperature: 0.7,
                max_tokens: 300,
                system_prompt: "You are a helpful assistant who writes concise, professional Git commit messages. Given code changes, write a commit message with a clear single-line summary followed by a more detailed description in Markdown that explains what was changed and why.".to_string(),
            },
        }
    }
}

impl Config {
    pub fn load(path: &Path) -> Result<Self> {
        let content = fs::read_to_string(path)
            .context(format!("Failed to read config file: {:?}", path))?;
        let config: Config = toml::from_str(&content)
            .context("Failed to parse config file")?;
        Ok(config)
    }

    pub fn save(&self, path: &Path) -> Result<()> {
        // Ensure the parent directory exists
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        
        let content = toml::to_string(self)?;
        fs::write(path, content)?;
        Ok(())
    }
}

pub fn get_config_path() -> Result<PathBuf> {
    let mut path = dirs::config_dir()
        .context("Failed to determine config directory")?;
    path.push("brocode");
    path.push("config.toml");
    Ok(path)
}
