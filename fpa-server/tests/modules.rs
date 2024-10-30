mod shared;

use anyhow::{Ok, Result};
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
    assert_eq!(response.status(), StatusCode::CREATED);
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

async fn by_id(token: &String, project: &Uuid, data: &Data) -> Result<()> {
    let response = reqwest::Client::new()
        .get(format!("{}/{}/modules/{}", URL, project, data.module))
        .bearer_auth(token)
        .send()
        .await?;
    assert_eq!(response.status(), StatusCode::OK); 


    let json = response.json::<Data>().await?;
    assert_eq!(json.module, data.module);
    assert_eq!(json.name, data.name);
    assert_eq!(json.description, data.description);  

    Ok(())
}

async fn list(token: &String, project: &Uuid) -> Result<()> {
    let response = reqwest::Client::new()
        .get(format!("{}/{}/modules", URL, project))
        .bearer_auth(token)
        .send()
        .await?;
    assert_eq!(response.status(), StatusCode::OK); 

    let json = response.json::<serde_json::Value>().await?;
    assert_eq!(json["pages"], json!(1));
    assert_eq!(json["index"], json!(1));
    assert_eq!(json["size"], json!(1));
    assert_eq!(json["records"], json!(1));    
    assert!(json["items"].is_array());
    assert_eq!(json["items"].as_array().unwrap().len(), 1);

    Ok(())
}

async fn update(token: &String, project: &Uuid, data: &Data) -> Result<()> {
    let body = json!({
        "name": data.name.clone(),
        "description": Some(String::from(MODULE_DESCRIPTION)),
    });
    let response = reqwest::Client::new()
        .put(format!("{}/{}/modules/{}", URL, project, data.module))
        .bearer_auth(token)
        .json(&body)
        .send()
        .await?;
    assert_eq!(response.status(), StatusCode::OK);

    let json = response.json::<Data>().await?;
    assert!(!json.module.is_nil());
    assert_eq!(json.name, MODULE_NAME);
    assert_eq!(json.description.unwrap(), MODULE_DESCRIPTION);

    Ok(())
}

async fn remove(token: &String, project: &Uuid, module: &Uuid) -> Result<()> {
    let response = reqwest::Client::new()
        .delete(format!("{}/{}/modules/{}", URL, project, module))
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

    remove(&token, &project, &data.module).await?;
    
    Ok(())
}