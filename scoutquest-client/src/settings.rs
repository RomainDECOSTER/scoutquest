use std::{fmt, path::Path};
use serde::Deserialize;
use config::{Config, ConfigError, File};

const CONFIG_FILE_PATH: &str = "config/default.toml";
const CONFIG_FILE_PREFIX: &str = "config/";

#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    pub uri: String,
    pub service_name: String,
}

/// Server settings
#[derive(Debug, Deserialize, Clone)]
pub struct Server {
    /// Port to run the server
    pub port: u16,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ScoutQuestConfig {
    pub scout_quest_config: Settings,
    /// Server settings
    pub server: Server,
}

/// Environment settings
#[derive(Clone, Debug, Deserialize)]
pub enum ENV {
    Development,
    Production,
}

/// Display the environment
impl fmt::Display for ENV {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ENV::Development => write!(f, "dev"),
            ENV::Production => write!(f, "prod"),
        }
    }
}

/// Convert a string to an environment
impl From<&str> for ENV {
    fn from(env: &str) -> Self {
        match env {
            "prod" => ENV::Production,
            _ => ENV::Development,
        }
    }
}

impl ScoutQuestConfig {
    pub fn new() -> Result<Self, ConfigError> {
        let env = std::env::var("RUN_ENV").unwrap_or_else(|_| "dev".into());
        let s = Config::builder()
            .add_source(File::with_name(CONFIG_FILE_PATH));
        let s = match Path::new(&format!("{}{}", CONFIG_FILE_PREFIX, env)).exists() {
            true => s.add_source(File::with_name(&format!("{}{}", CONFIG_FILE_PREFIX, env))),
            false => s,
        };
        let s = s.build()?;
        s.try_deserialize()
    }
}