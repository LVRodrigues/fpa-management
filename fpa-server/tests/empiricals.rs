mod shared;

use anyhow::Result;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use serde_json::json;
use shared::{
    selects,
    tokens::{self, Tenant},
    PASSWORD, URL, USERNAME,
};
use uuid::Uuid;

const PROCUCTIVITY: &str = "Productivity";
const DEPLOYMENT: &str = "Deployment";

#[derive(Debug, Serialize, Deserialize)]
struct Data {
    empirical: String,
    value: i32,
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
    const VALUE: i32 = 50;
    let body = Data {
        empirical: String::from(DEPLOYMENT),
        value: VALUE,
    };
    let response = reqwest::Client::new()
        .put(format!("{}/{}/empiricals", URL, &project))
        .bearer_auth(&token)
        .json(&body)
        .send()
        .await?;
    assert_eq!(response.status(), StatusCode::OK);

    let data = response.json::<Data>().await?;
    assert_eq!(data.empirical, DEPLOYMENT);
    assert_eq!(data.value, VALUE);

    Ok(())
}

async fn update_productivity_error(token: &String, project: &Uuid) -> Result<()> {
    const VALUE: i32 = 60;
    let body = Data {
        empirical: String::from(PROCUCTIVITY),
        value: VALUE,
    };
    let response = reqwest::Client::new()
        .put(format!("{}/{}/empiricals", URL, &project))
        .bearer_auth(&token)
        .json(&body)
        .send()
        .await?;
    assert_eq!(response.status(), StatusCode::NOT_ACCEPTABLE);

    Ok(())
}

async fn update_deployment_error(token: &String, project: &Uuid) -> Result<()> {
    const VALUE: i32 = 160;
    let body = Data {
        empirical: String::from(DEPLOYMENT),
        value: VALUE,
    };
    let response = reqwest::Client::new()
        .put(format!("{}/{}/empiricals", URL, &project))
        .bearer_auth(&token)
        .json(&body)
        .send()
        .await?;
    assert_eq!(response.status(), StatusCode::NOT_ACCEPTABLE);

    Ok(())
}

#[tokio::test]
async fn execute() -> Result<()> {
    let token = tokens::request_token(USERNAME, PASSWORD, Tenant::TENANT_DEFAULT).await?;
    assert!(!token.is_empty());

    let project = selects::project(&token).await?;

    list(&token, &project).await?;

    update(&token, &project).await?;
    update_productivity_error(&token, &project).await?;
    update_deployment_error(&token, &project).await?;

    Ok(())
}
