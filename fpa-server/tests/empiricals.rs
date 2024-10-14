mod shared;

use anyhow::Result;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use serde_json::json;
use shared::{selects, tokens::{self, Tenant}, URL};
use uuid::Uuid;

const USERNAME: &str = "user";
const PASSWORD: &str = "fpa-pass";

const EMPIRICAL: &str = "Deployment";
const VALUE: u64 = 50;

#[derive(Debug, Serialize, Deserialize)]
struct Data {
    empirical: String,
    value: u64
}

async fn list(token: &String, project: &Uuid) -> Result<()> {
    let response = reqwest::Client::new()
        .get(format!("{}/{}/empiricals", URL, &project))
        .bearer_auth(&token)
        .send()
        .await?;
    assert_eq!(response.status(), StatusCode::OK);

    let json = response.json::<serde_json::Value>().await?;
    assert_eq!(json["pages"], json!(1));
    assert_eq!(json["index"], json!(1));
    assert_eq!(json["size"], json!(5));
    assert_eq!(json["records"], json!(5));
    assert!(json["items"].is_array());
    assert_eq!(json["items"].as_array().unwrap().len(), 5);

    Ok(())
}

async fn update(token: &String, project: &Uuid) -> Result<()> {
    let body = Data {
        empirical: String::from(EMPIRICAL),
        value: VALUE
    };
    let response = reqwest::Client::new()
        .put(format!("{}/{}/empiricals", URL, &project))
        .bearer_auth(&token)
        .json(&body)
        .send()
        .await?;
    assert_eq!(response.status(), StatusCode::OK);    

    let data = response.json::<Data>().await?;
    assert_eq!(data.empirical, EMPIRICAL);
    assert_eq!(data.value, VALUE);

    Ok(())
}

#[tokio::test]
async fn execute() -> Result<()> {
    let token = tokens::request_token(USERNAME, PASSWORD, Tenant::TENANT_DEFAULT).await?;
    assert!(!token.is_empty());

    let project = selects::project(&token).await?;

    list(&token, &project).await?;

    update(&token, &project).await?;

    Ok(())
}
