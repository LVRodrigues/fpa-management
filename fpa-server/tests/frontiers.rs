mod shared;

use anyhow::{Ok, Result};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use serde_json::json;
use shared::{
    selects,
    tokens::{self, Tenant},
    PASSWORD, URL, USERNAME,
};
use uuid::Uuid;

const FRONTIER_NAME: &str = "Frontier Test";
const FRONTIER_DESCRIPTION: &str = "Descrição longa da nova fronteira de teste.";

#[derive(Debug, Serialize, Deserialize)]
struct Data {
    frontier: Uuid,
    name: String,
    description: Option<String>,
}

async fn create(token: &String, project: &Uuid) -> Result<Data> {
    let body = json!({
        "name": FRONTIER_NAME
    });
    let response = reqwest::Client::new()
        .post(format!("{}/{}/frontiers", URL, project))
        .bearer_auth(token)
        .json(&body)
        .send()
        .await?;
    assert_eq!(response.status(), StatusCode::CREATED);
    assert!(response.headers().get("location").is_some());

    let data = response.json::<Data>().await?;
    assert!(!data.frontier.is_nil());
    assert_eq!(data.name, FRONTIER_NAME);
    assert_eq!(data.description, None);

    Ok(data)
}

async fn create_duplicate(token: &String, project: &Uuid) -> Result<()> {
    let body = json!({
        "name": FRONTIER_NAME
    });
    let response = reqwest::Client::new()
        .post(format!("{}/{}/frontiers", URL, project))
        .bearer_auth(token)
        .json(&body)
        .send()
        .await?;
    assert_eq!(response.status(), StatusCode::CONFLICT);

    Ok(())
}

async fn by_id(token: &String, project: &Uuid, data: &Data) -> Result<()> {
    let response = reqwest::Client::new()
        .get(format!("{}/{}/frontiers/{}", URL, project, data.frontier))
        .bearer_auth(token)
        .send()
        .await?;
    assert_eq!(response.status(), StatusCode::OK);

    let json = response.json::<Data>().await?;
    assert_eq!(json.frontier, data.frontier);
    assert_eq!(json.name, data.name);
    assert_eq!(json.description, data.description);

    Ok(())
}

async fn list(token: &String, project: &Uuid) -> Result<()> {
    let response = reqwest::Client::new()
        .get(format!("{}/{}/frontiers", URL, project))
        .bearer_auth(token)
        .send()
        .await?;
    assert_eq!(response.status(), StatusCode::OK);

    let json = response.json::<serde_json::Value>().await?;
    assert_eq!(json["pages"], json!(1));
    assert_eq!(json["index"], json!(1));
    assert_eq!(json["size"], json!(2));
    assert_eq!(json["records"], json!(2));
    assert!(json["items"].is_array());
    assert_eq!(json["items"].as_array().unwrap().len(), 2);

    Ok(())
}

async fn update(token: &String, project: &Uuid, data: &Data) -> Result<()> {
    let body = json!({
        "name": data.name.clone(),
        "description": Some(String::from(FRONTIER_DESCRIPTION)),
    });
    let response = reqwest::Client::new()
        .put(format!("{}/{}/frontiers/{}", URL, project, data.frontier))
        .bearer_auth(token)
        .json(&body)
        .send()
        .await?;
    assert_eq!(response.status(), StatusCode::OK);

    let json = response.json::<Data>().await?;
    assert!(!json.frontier.is_nil());
    assert_eq!(json.name, FRONTIER_NAME);
    assert_eq!(json.description.unwrap(), FRONTIER_DESCRIPTION);

    Ok(())
}

async fn remove(token: &String, project: &Uuid, frontier: &Uuid) -> Result<()> {
    let response = reqwest::Client::new()
        .delete(format!("{}/{}/frontiers/{}", URL, project, frontier))
        .bearer_auth(token)
        .send()
        .await?;
    assert_eq!(response.status(), StatusCode::NO_CONTENT);

    Ok(())
}

#[tokio::test]
async fn execute() -> Result<()> {
    let token = tokens::request_token(USERNAME, PASSWORD, Tenant::TENANT_DEFAULT).await?;
    assert!(!token.is_empty());

    let project = selects::project(&token).await?;

    let data = create(&token, &project).await?;
    create_duplicate(&token, &project).await?;

    by_id(&token, &project, &data).await?;

    list(&token, &project).await?;

    update(&token, &project, &data).await?;

    remove(&token, &project, &data.frontier).await?;

    Ok(())
}
