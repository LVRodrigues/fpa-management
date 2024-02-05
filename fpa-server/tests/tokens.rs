#![allow(unused)]

use std::{collections::HashMap, error::Error};
use axum::Json;
use reqwest::StatusCode;
use url::form_urlencoded;

use anyhow::Result;

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Deserialize)]
struct AccessTokenResponse {
    access_token: String,
    expires_in: u64,
    refresh_expires_in: u64,
    refresh_token: String,
    token_type: String,
    #[serde(rename="not-before-policy")]
    not_before_policy: u64,
    session_state: String,
    scope: String,
}

#[allow(non_camel_case_types)]
pub enum Tenant {
    TENANT_01,
    TENANT_02,
}

pub async fn request_token(user: &str, password: &str, tenant: Tenant) -> Result<String> {
    let (realm, secret) = match tenant {
        Tenant::TENANT_01 => ("tenant-01", "jKQO0Pxb1gFrSz64iUgqlgsoANs86d31"),
        Tenant::TENANT_02 => ("tenant-02", "mUyu1Jd9VKIWCxrHkl00NauuAxzO7KCP"),
    };
    let mut params = HashMap::new();
    params.insert("grant_type", "password");
    params.insert("client_id", "fpa-management");
    params.insert("client_secret", secret);
    params.insert("username", user);
    params.insert("password", password);
    let body = form_urlencoded::Serializer::new(String::new())
        .extend_pairs(params.iter())
        .finish();
    let response: AccessTokenResponse = reqwest::Client::new()
        .post(format!("http://localhost:8080/realms/{realm}/protocol/openid-connect/token"))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await?
        .json()
        .await?;

    Ok(response.access_token)
}