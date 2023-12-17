use std::collections::HashMap;

use jsonwebtoken::jwk::Jwk;
use serde::{Deserialize, Serialize};

use crate::{configuration::Configuration, error::Error};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Key {
    pub kid: String,
    pub kty: String,
    pub alg: String,
    #[serde(rename = "use")]
    pub use_for: String,
    pub n: String,
    pub e: String,
    pub x5c: Vec<String>,
    pub x5t: String,
    #[serde(rename = "x5t#S256")]
    pub x5t_s256: String,
}

impl Key {
    pub fn to_jwk(&self) -> Jwk {
        let jwk_str = serde_json::to_string(&self).expect("Failed to serialize JWK");
        serde_json::from_str(&jwk_str).expect("Failed to deserialize JWK")
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct Keys {
    #[serde(rename = "keys")]
    items: Vec<Key>,
}

static mut KEYS: Option<HashMap<String, Key>> = None;

async fn request_jwks(tenant: String) -> Result<Keys, Error> {
    let jwks: Keys = reqwest::Client::new()
        .get(tenant)
        .send()
        .await?
        .json()
        .await?;

    Ok(jwks)
}

pub async fn prepare(config: Configuration) -> Result<(), Error> {
    let keys = unsafe { KEYS.get_or_insert_with(|| HashMap::new()) };

    for jwks in &config.jwks {
        let ks = request_jwks(jwks.clone()).await?;
        for key in ks.items {
            keys.insert(key.kid.clone(), key);
        }
    }

    Ok(())
}

pub fn key(kid: String) -> Result<Key, Error> {
    let keys = unsafe { KEYS.get_or_insert_with(|| HashMap::new()) };

    let key = keys.get(&kid).cloned();
    let key = match key {
        Some(k) => k,
        None => return Err(Error::KeyNotFound),
    };

    Ok(key)
}
