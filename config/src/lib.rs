// Loads all the user configurations in ~/.config/chat

use serde::Deserialize;
use config::{Config, File};
use dirs::config_dir;
use once_cell::sync::OnceCell;

#[derive(Debug, Deserialize)]
pub struct GeneralConfig {
    pub name: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ServerConfig {
    pub server_ip: String,
    pub server_port: String,
}

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    #[serde(rename = "general")]
    pub general: GeneralConfig,
    #[serde(rename = "server")]
    pub server: ServerConfig,
}

static CONFIG: OnceCell<AppConfig> = OnceCell::new();

pub fn get_config() -> &'static AppConfig {
    CONFIG.get_or_init(|| {
        let mut builder = Config::builder();

        let config_path = config_dir().unwrap_or_else(|| {
            eprintln!("Couldn't find the config file");
            std::process::exit(1);
        }).join("chat").join("config.ini");

        builder = builder.add_source(File::from(config_path));

        let config = builder.build().unwrap_or_else(|err| {
            eprintln!("Failed to build the config: {}", err);
            std::process::exit(1);
        });

        config.try_deserialize().unwrap_or_else(|err| {
            eprintln!("Failed to parse the config: {}", err);
            std::process::exit(1);
        })
    })
}