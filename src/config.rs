use std::collections::HashMap;
use std::path::PathBuf;

use config::{Config, File};
use home::home_dir;

pub struct SlangConfig {
    data: Config
}

fn get_default_config() -> Option<PathBuf> {
    let home = home_dir();

    match home {
        Some(mut default_config) => {
            default_config.push(".slang.toml");

            match default_config.exists() {
                true => Some(default_config),
                false => None
            }
        },
        None => None
    }
}

impl SlangConfig {
    /// Merge config from file
    fn merge_file(&mut self, path: PathBuf) -> Result<(), config::ConfigError> {
        if path.exists() {
            self.data.merge(File::from(path))?;
        }

        Ok(())
    }

    /// Return `sources` key from config data.
    pub fn get_sources(&self, context: Option<String>) -> Result<HashMap<String, String>, config::ConfigError> {
        let mut sources: HashMap<String, String> = self.data.get("sources")?;

        // Limit sources by provided context
        if let Some(context) = context {
            sources.retain(|source, _| source == &context);
        }

        Ok(sources)
    }

    /// Get config
    pub fn new(config_file: Option<PathBuf>) -> Result<Self, config::ConfigError> {
        let mut config = Self {
            data: Config::default()
        };

        match config_file.or_else(get_default_config) {
            // Path to default acronyms awnd potential user config
            Some(user_config) => config.merge_file(user_config)?,
            // Merge in defaults to provide functionality out of the box.
            None => config.merge_file(PathBuf::from("defaults.toml"))?
        }

        Ok(config)
    }
}