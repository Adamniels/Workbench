use serde::Deserialize;
use anyhow::{Context, Result};
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
pub struct Config{
    pub settings: Option<Settings>,
    pub projects: Vec<Project>
}


#[derive(Debug, Deserialize)]
pub struct Settings {
    pub editor: String,
    pub terminal: String,
    pub shell: String
}


#[derive(Debug, Deserialize, Clone)]
pub struct Project{
    pub name: String,
    pub path: String,
    pub tags: Vec<String>,
    pub default_action: String,
    pub actions: Vec<Action>
}

#[derive(Debug, Deserialize, Clone)]
pub struct Action{
    pub key: String,
    pub name: String,
    pub cmd: String
}

fn config_path() -> Result<PathBuf> {
    let base = if let Ok(xdg) = std::env::var("XDG_CONFIG_HOME") {
        PathBuf::from(xdg)
    } else {
        let home = std::env::var("HOME")
            .context("HOME or XDG_CONFIG_HOME must be set")?;
        PathBuf::from(home).join(".config")
    };
    Ok(base.join("workbench").join("config.toml"))
}

pub fn load() -> Result<Config> {
    let path = config_path()?;
    let content = std::fs::read_to_string(&path).with_context(|| format!("Could not read config from {}", path.display()))?;
    let config: Config = toml::from_str(&content).context("Failed to parse config.toml")?;
    Ok(config)
}
