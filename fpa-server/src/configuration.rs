use std::{path::Path, str::FromStr};

use axum::http::uri::Scheme;
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
pub struct Empiricals {
    pub productivity: i32,
    pub coordination: i32,
    pub deployment: i32,
    pub planning: i32,
    pub testing: i32,
}

#[derive(Debug, Clone)]
pub struct Configuration {
    pub scheme: Scheme,
    pub authority: String,
    pub port: u16,
    pub jwks: Vec<String>,
    pub database: ConfigurationDatabase,
    pub empiricals: Empiricals,
}

pub fn prepare() -> Configuration {
    println!("==> {:<12} - prepare", "CONFIG");
    let settings = Config::builder()
        .add_source(File::from(Path::new("config.yaml")))
        .build()
        .unwrap();

    let scheme: String = settings.get("scheme").unwrap();
    Configuration {
        scheme: Scheme::from_str(scheme.as_str()).unwrap(),
        authority: settings.get("authority").unwrap(),
        port: settings.get("port").unwrap(),
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
        empiricals: Empiricals {
            productivity: settings.get("empiricals.productivity").unwrap(),
            coordination: settings.get("empiricals.coordination").unwrap(),
            deployment: settings.get("empiricals.deployment").unwrap(),
            planning: settings.get("empiricals.planning").unwrap(),
            testing: settings.get("empiricals.testing").unwrap(),
        },
    }
}
