use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    pub server: ServerConfig,
    #[allow(dead_code)]
    pub database: DatabaseConfig,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Deserialize, Clone)]
pub struct DatabaseConfig {
    #[allow(dead_code)]
    pub url: String,
}

impl Settings {
    pub fn new() -> Result<Self, config::ConfigError> {
        let settings = config::Config::builder()
            .add_source(config::Environment::default().separator("__"))
            .set_default("server.host", "0.0.0.0")?
            .set_default("server.port", "3000")?
            .set_default("database.url", "memory")?
            .build()?;
        
        settings.try_deserialize()
    }
}