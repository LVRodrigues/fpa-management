use std::path::Path;

use config::{Config, File};

#[derive(Debug)]
pub struct ConfigurationDatabase {
    pub engine: String,
    pub server: String,
    pub port: u16,
    pub username: String,
    pub password: String,
}

#[derive(Debug)]
pub struct Configuration {
    pub jwks: Vec<String>,
    pub database: ConfigurationDatabase,
}

pub fn prepare() -> Configuration {
    let settings = Config::builder()
        .add_source(File::from(Path::new("config.yaml")))
        .build()
        .unwrap();

    Configuration {
        jwks: settings.get("jwks").unwrap(),
        database: ConfigurationDatabase {
            engine: settings.get("database.engine").unwrap(),
            server: settings.get("database.server").unwrap(),
            port: settings.get("database.port").unwrap(),
            username: settings.get("database.username").unwrap(),
            password: settings.get("database.password").unwrap(),
        },
    }
}