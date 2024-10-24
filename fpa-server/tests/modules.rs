mod shared;

use anyhow::Result;
use shared::{selects, tokens::{self, Tenant}, URL, USERNAME, PASSWORD};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use serde_json::json;
use reqwest::StatusCode;

const MODULE_NAME: &str = "Module Test";
const MODULE_DESCRIPTION: &str = "Descrição longa do novo módulo de teste.";

#[derive(Debug, Serialize, Deserialize)]
struct Data {
    module: Uuid,
    name: String,
    description: Option<String>,
}

async fn create(token: &String, project: &Uuid) -> Result<Data> {
    let body = json!({
        "name": MODULE_NAME
    });
    let response = reqwest::Client::new()
        .post(format!("{}/{}/modules", URL, project))
        .bearer_auth(token)
        .json(&body)
        .send()
        .await?;
    assert!(response.status() == StatusCode::CREATED);
    assert!(response.headers().get("location").is_some());

    let data = response.json::<Data>().await?;
    assert!(!data.module.is_nil());
    assert_eq!(data.name, MODULE_NAME);
    assert_eq!(data.description, None);

    Ok(data)
}

async fn create_duplicate(token: &String, project: &Uuid) -> Result<()> {
    let body = json!({
        "name": MODULE_NAME
    });
    let response = reqwest::Client::new()
        .post(format!("{}/{}/modules", URL, project))
        .bearer_auth(token)
        .json(&body)
        .send()
        .await?;
    assert_eq!(response.status(), StatusCode::CONFLICT);

    Ok(())
}

#[tokio::test]
async fn execute() -> Result<()> {
    let token = tokens::request_token(USERNAME, PASSWORD, Tenant::TENANT_DEFAULT).await?;
    assert!(!token.is_empty());    

    let project = selects::project(&token).await?;

    let _data = create(&token, &project).await?;
    create_duplicate(&token, &project).await?;
    
    Ok(())
}