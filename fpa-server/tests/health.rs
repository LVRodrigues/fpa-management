use anyhow::Result;
use reqwest::StatusCode;

use crate::tokens::Tenant;

mod tokens;

#[tokio::test]
async fn health() -> Result<()> {
    let token = tokens::request_token("user", "fpa-pass", Tenant::TENANT_DEFAULT).await?;

    let response = reqwest::Client::new()
        .get("http://localhost:5000/api/health")
        .bearer_auth(token)
        .send()
        .await?;
    println!("Status: {}", response.status());
    assert!(response.status() == StatusCode::NO_CONTENT);

    Ok(())
}