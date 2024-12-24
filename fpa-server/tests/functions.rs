mod shared;

use anyhow::{Ok, Result};
use shared::{selects, tokens::{self, Tenant}, URL, USERNAME, PASSWORD};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use serde_json::json;
use reqwest::StatusCode;

async fn list(token: &String, project: &Uuid, module: &Uuid) -> Result<()> {
    let response = reqwest::Client::new()
        .get(format!("{}/{}/modules/{}/functions", URL, project, module))
        .bearer_auth(token)
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

#[tokio::test]
async fn execute() -> Result<()> {
    let token = tokens::request_token(USERNAME, PASSWORD, Tenant::TENANT_DEFAULT).await?;
    assert!(!token.is_empty());    

    let project = selects::project(&token).await?;
    let module = selects::module(&token, &project).await?;

    list(&token, &project, &module).await?;
    
    Ok(())
}