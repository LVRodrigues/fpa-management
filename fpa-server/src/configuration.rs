use std::{path::Path, collections::HashMap};

use config::{Config, File};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Configuration {
    pub jwks_tenant_01: String,
    pub jwks_tenant_02: String,
}

impl From<HashMap<std::string::String, std::string::String>> for Configuration {
    fn from(value: HashMap<std::string::String, std::string::String>) -> Self {
        Configuration { 
            jwks_tenant_01: value.get("jwks_tenant_01").unwrap().clone(), 
            jwks_tenant_02: value.get("jwks_tenant_02").unwrap().clone(),
        }
    }
}

pub fn prepare() -> Configuration {
    let settings = Config::builder()
        .add_source(File::from(Path::new("config.ini")))
        .build()
        .unwrap();

    let config_map: HashMap<String, String> = settings
        .try_deserialize::<HashMap<String, String>>()
        .unwrap();

    let configuration: Configuration = config_map.try_into().unwrap();
    
    configuration
}