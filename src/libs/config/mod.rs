mod tools;

use std::env;
use std::path::PathBuf;
use serde_yaml_ng::Value;

use serde::{Deserialize, Serialize};

// load default config from file
const DEFAULT_CONFIG_YAML: &str = include_str!("config.yaml");

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum State {
    New,
    Enabled,
    Disabled,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct YamlConfig {
    pub url: String,
    pub files: Vec<FileConfig>,
    pub spells: Vec<SpellConfig>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct FileConfig {
    pub name: String,
    pub version: String,
    pub state: State,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SpellConfig {
    pub name: String,
    pub url: String,
    pub path: Option<PathBuf>,
    pub state: State,
}

#[derive(Deserialize, Debug)]
#[serde(default)]
pub struct Config {
    // mutable config
    pub log_level: String,

    // base directories
    #[serde(skip)]
    pub home_dir: PathBuf,
    #[serde(skip)]
    pub work_dir: PathBuf,

    // sub directories
    #[serde(skip)]
    pub base_dir: PathBuf,
    #[serde(skip)]
    pub bin_dir: PathBuf,
    #[serde(skip)]
    pub conf_dir: PathBuf,
    #[serde(skip)]
    pub repo_dir: PathBuf,
    #[serde(skip)]
    pub venv_dir: PathBuf,
    #[serde(skip)]
    pub pre_commit_home: PathBuf,

    // YAML based configs
    #[serde(skip)]
    pub url: String,
    #[serde(skip)]
    pub files: Vec<FileConfig>,
    #[serde(skip)]
    pub spells: Vec<SpellConfig>,
}

// Config default loader called by serde(default)
impl Default for Config {
    fn default() -> Self {
        let home_dir = env::home_dir()
            .expect("unable to determine home directory");

        let work_dir = env::current_dir()
            .expect("failed to get current directory");

        let base_dir = home_dir.join(".spellbook");
        let bin_dir = base_dir.join("bin");
        let conf_dir = base_dir.join("configs");

        let repo_name = work_dir
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("default");
        let repo_dir = conf_dir.join(repo_name);

        let _config: YamlConfig = serde_yaml_ng::from_str(DEFAULT_CONFIG_YAML)
            .expect("Failed to parse embedded config.yaml");

        Self {
            log_level: "info".to_string(),
            home_dir,
            work_dir,
            base_dir,
            bin_dir,
            conf_dir,
            venv_dir: repo_dir.join(".venv"),
            pre_commit_home: repo_dir.join(".pre-commit"),
            repo_dir,
            url: _config.url,
            files: _config.files,
            spells: _config.spells,
        }
    }
}

// Config main implementation
impl Config {
    pub fn merge(&mut self) -> Result<(), Box<dyn std::error::Error>> {

        let config_path = self.repo_dir.join("config.yaml");

        if !config_path.exists() {
            log::debug!("user config not found, using default");
            return Ok(());
        }

        let local = std::fs::read_to_string(&config_path)?;
        let _config: YamlConfig = serde_yaml_ng::from_str(&local)?;

        // set from local config
        self.files = _config.files;
        self.spells = _config.spells;

        log::info!("merged config from: {}", config_path.display());
        Ok(())
    }
}

/*
Load configuration both from .env file
and from environment, configuration params
should be prefixed with `APP_` string.
*/
pub fn load() -> Config {
    dotenvy::dotenv().ok();
    envy::prefixed("SPELL_").from_env().unwrap()
}

#[cfg(test)]
mod tests;
