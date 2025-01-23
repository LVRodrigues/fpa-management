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

const FACTOR: &str = "MultipleSites";
const INFLUENCE: &str = "Significant";

#[derive(Debug, Deserialize, Serialize)]
struct Data {
    factor: String,
    influence: String,
}

async fn list(token: &String, project: &Uuid, frontier: &Uuid) -> Result<()> {
    let response = reqwest::Client::new()
        .get(format!(
            "{}/{}/frontiers/{}/factors",
            URL, project, frontier
        ))
        .bearer_auth(&token)
        .send()
        .await?;
    assert_eq!(response.status(), StatusCode::OK);

    let json = response.json::<serde_json::Value>().await?;
    assert_eq!(json["pages"], json!(1));
    assert_eq!(json["index"], json!(1));
    assert_eq!(json["size"], json!(14));
    assert_eq!(json["records"], json!(14));
    assert!(json["items"].is_array());
    assert_eq!(json["items"].as_array().unwrap().len(), 14);

    Ok(())
}

async fn update(token: &String, project: &Uuid, frontier: &Uuid) -> Result<()> {
    let body = Data {
        factor: String::from(FACTOR),
        influence: String::from(INFLUENCE),
    };
    let response = reqwest::Client::new()
        .put(format!(
            "{}/{}/frontiers/{}/factors",
            URL, project, frontier
        ))
        .bearer_auth(&token)
        .json(&body)
        .send()
        .await?;
    assert_eq!(response.status(), StatusCode::OK);

    let data = response.json::<Data>().await?;
    assert_eq!(data.factor, FACTOR);
    assert_eq!(data.influence, INFLUENCE);
    Ok(())
}

#[tokio::test]
async fn execute() -> Result<()> {
    let token = tokens::request_token(USERNAME, PASSWORD, Tenant::TENANT_DEFAULT).await?;
    assert!(!token.is_empty());

    let project = selects::project(&token).await?;
    let frontier = selects::frontier(&token, &project).await?;

    list(&token, &project, &frontier).await?;

    update(&token, &project, &frontier).await?;

    Ok(())
}
