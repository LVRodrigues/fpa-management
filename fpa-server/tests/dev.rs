#![allow(unused)]

use std::{collections::HashMap, error::Error};
use axum::Json;
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

async fn request_token() -> Result<String> {
    let mut params = HashMap::new();
    params.insert("grant_type", "password");
    params.insert("client_id", "fpa-management");
    params.insert("client_secret", "jKQO0Pxb1gFrSz64iUgqlgsoANs86d31");
    params.insert("username", "user");
    params.insert("password", "fpa-pass");
    let body = form_urlencoded::Serializer::new(String::new())
        .extend_pairs(params.iter())
        .finish();
    let response: AccessTokenResponse = reqwest::Client::new()
        .post("http://localhost:8080/realms/tenant-01/protocol/openid-connect/token")
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await?
        .json()
        .await?;

    Ok(response.access_token)
}

#[tokio::test]
async fn dev() -> Result<()> {
    let token = request_token()
        .await?;

    let response = reqwest::Client::new()
        .get("http://localhost:5000/api/hello")
        .bearer_auth(token)
        .send()
        .await?
        .text()
        .await?;

    println!("{:?}", response);

    Ok(())
}