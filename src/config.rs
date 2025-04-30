use config::{Config, File};
use log::error;
use serde::{Deserialize, Serialize};

use crate::log_to_file;

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub(crate) struct ClientConfig {
    pub(crate) name: String,
    pub(crate) gitlab_host: String,
    pub(crate) gitlab_token: String,
    pub(crate) author_email: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub(crate) struct AppConfig {
    pub(crate) clients: Vec<ClientConfig>,
}

pub(crate) fn get_config() -> AppConfig {
    // First, try to load from the executable directory
    let exe_dir_config = get_config_from_exe_dir();
    if exe_dir_config.is_some() {
        let config = exe_dir_config.unwrap();

        if config.clients.is_empty() {
            let _ = log_to_file("Config loaded but contains no clients!");
            error!("Config loaded but contains no clients!");
        }

        config
    } else {
        let _ = log_to_file("Could not load configuration file");
        error!("Could not load configuration file");
        std::process::exit(1);
    }
}

/// Try to load config from the executable directory
fn get_config_from_exe_dir() -> Option<AppConfig> {
    let exe_path = std::env::current_exe().ok()?;
    let exe_dir = exe_path.parent()?;
    let _ = log_to_file(&format!(
        "Attempting to load config from executable directory: {}",
        exe_dir.display()
    ));
    let config_path = exe_dir.join("config.toml");
    let _ = log_to_file(&format!("Config path: {}", config_path.display()));

    if !config_path.exists() {
        return None;
    }

    match Config::builder()
        .add_source(File::from(config_path))
        .build()
    {
        Ok(config) => match config.try_deserialize::<AppConfig>() {
            Ok(app_config) => Some(app_config),
            Err(e) => {
                let _ = log_to_file(&format!("Failed to deserialize config: {}", e));
                error!("Failed to deserialize config: {}", e);
                None
            }
        },
        Err(e) => {
            let _ = log_to_file(&format!("Failed to build config: {}", e));
            error!("Failed to build config: {}", e);
            None
        }
    }
}
