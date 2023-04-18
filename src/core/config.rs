use std::fs;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct OidcConfig {
    pub base_url: String,
    pub audience: String,
    pub client_id: String,
}

#[derive(Deserialize)]
pub struct Config {
    pub oidc: OidcConfig,
    pub app_path: String,
}

impl Config {
    pub fn load() -> Self {
        let file = fs::File::open("config.json").expect("Failed to open config.json");
        return serde_json::from_reader(file).expect("Malformed JSON in config.json");
    }
}
