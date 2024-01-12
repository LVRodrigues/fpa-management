use std::path::Path;

use config::{Config, File};

#[derive(Debug, Clone)]
pub struct ConfigurationDatabase {
    pub engine: String,
    pub server: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub name: String,
    pub connections_max: u32,
    pub connections_min: u32,
    pub timeout_connect: u64,
    pub timeout_acquire: u64,
    pub timeout_idle: u64,
    pub lifetime: u64,
}

#[derive(Debug, Clone)]
pub struct Configuration {
    pub jwks: Vec<String>,
    pub database: ConfigurationDatabase,
}

pub fn prepare() -> Configuration {
    println!("==> {:<12} - prepare", "CONFIG");
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
            name: settings.get("database.name").unwrap(),
            connections_max: settings.get("database.connections_max").unwrap(),
            connections_min: settings.get("database.connections_min").unwrap(),
            timeout_connect: settings.get("database.timeout_connect").unwrap(),
            timeout_acquire: settings.get("database.timeout_acquire").unwrap(),
            timeout_idle: settings.get("database.timeout_idle").unwrap(),
            lifetime: settings.get("database.lifetime").unwrap(),
        },
    }
}