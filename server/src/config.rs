use std::fmt;
use config::{Config, ConfigError, File};
use serde::Deserialize;

const CONFIG_FILE_PATH: &str = "config/default.toml";
const CONFIG_FILE_PREFIX: &str = "config/";

/// Logger settings
#[derive(Clone, Debug, Deserialize)]
pub struct Logger {
    /// Log level by default is DEBUG
    pub level: String
}

/// Default logger settings
impl Default for Logger {
    fn default() -> Self {
        Self {
            level: "DEBUG".into()
        }
    }
}

/// Server settings
#[derive(Debug, Deserialize, Clone)]
pub struct Server {
    /// Port to run the server
    pub port: u16,
}

/// Application settings
#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    /// Logger settings
    pub logger: Logger,
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

/// Load settings from configuration files
impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let env = std::env::var("RUN_ENV").unwrap_or_else(|_| "dev".into());
        let s = Config::builder()
            .add_source(File::with_name(CONFIG_FILE_PATH))
            .add_source(File::with_name(&format!("{}{}", CONFIG_FILE_PREFIX, env)))
            .build()?;
        s.try_deserialize()
    }
}