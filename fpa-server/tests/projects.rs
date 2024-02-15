use anyhow::Result;
use reqwest::StatusCode;
use serial_test::serial;
use crate::tokens::Tenant;
use serde_json::json;

mod tokens;

#[tokio::test]
#[serial]
async fn list() -> Result<()> {
    let token = tokens::request_token("user", "fpa-pass", Tenant::TENANT_DEFAULT).await?;

    let response = reqwest::Client::new()
        .get("http://localhost:5000/api/projects")
        .bearer_auth(token)
        .send()
        .await?;
    assert_eq!(response.status(), StatusCode::OK);

    let json = response.json::<serde_json::Value>().await?;
    assert_eq!(*json.get("total").unwrap(), json!(10));
    assert_eq!(*json.get("index").unwrap(), json!(1));
    assert_eq!(*json.get("size").unwrap(), json!(10));
    assert_eq!(*json.get("records").unwrap(), json!(100));
    assert!(json.get("items").unwrap().is_array());
    assert_eq!(json.get("items").unwrap().as_array().unwrap().len(), 10);

    Ok(())
}

#[tokio::test]
#[serial]
async fn create() -> Result<()> {
    let token = tokens::request_token("user", "fpa-pass", Tenant::TENANT_DEFAULT).await?;

    let response = reqwest::Client::new()
        .post("http://localhost:5000/api/projects?name=Running%20Test")
        .bearer_auth(token)
        .send()
        .await?;
    assert!(response.status() == StatusCode::CREATED);
    assert!(response.content_length().unwrap() == 0);
    assert!(response.headers().get("location").is_some());

    Ok(())
}