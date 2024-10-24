mod shared;

use anyhow::Result;
use reqwest::StatusCode;
use sea_orm::prelude::DateTimeWithTimeZone;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use shared::{tokens::{self, Tenant}, URL, USERNAME, PASSWORD};
use serde_json::json;

const PROJECT_NAME: &str = "Running-Test";
const PROJECT_DESCRIPTION: &str = "Long project description for test";

#[derive(Debug, Deserialize, Serialize)]
struct Data {
    project: Uuid,
    name: String,
    description: Option<String>,
    time: DateTimeWithTimeZone,
    user: Uuid,    
}

impl PartialEq for Data {
    fn eq(&self, other: &Self) -> bool {
        self.project == other.project && self.name == other.name && self.time == other.time && self.user == other.user
    }
}

async fn list(token: &String) -> Result<()> {
    let response = reqwest::Client::new()
        .get(URL)
        .bearer_auth(token)
        .send()
        .await?;
    assert_eq!(response.status(), StatusCode::OK);

    let json = response.json::<serde_json::Value>().await?;
    assert_eq!(json["pages"], json!(10));
    assert_eq!(json["index"], json!(1));
    assert_eq!(json["size"], json!(10));
    assert_eq!(json["records"], json!(100));    
    assert!(json["items"].is_array());
    assert_eq!(json["items"].as_array().unwrap().len(), 10);
    Ok(())
}

async fn create(token: &String) -> Result<Data> {
    let body = json!({
        "name": PROJECT_NAME,
        "description": PROJECT_DESCRIPTION,
    });
    let response = reqwest::Client::new()
        .post(URL)
        .bearer_auth(token)
        .json(&body)
        .send()
        .await?;
    assert!(response.status() == StatusCode::CREATED);
    assert!(response.headers().get("location").is_some());

    let data = response.json::<Data>().await?;
    assert!(!data.project.is_nil());
    assert_eq!(data.name, PROJECT_NAME);
    assert_eq!(data.description, Some(String::from(PROJECT_DESCRIPTION)));
    assert!(!data.user.is_nil());

    Ok(data)
}

async fn find_by_id(token: &String, data: &Data) -> Result<()> { 
    let response = reqwest::Client::new()
        .get(format!("{}/{}", URL, data.project))
        .bearer_auth(token)
        .send()
        .await?;
    assert_eq!(response.status(), StatusCode::OK);

    let other = response.json::<Data>().await?;
    assert_eq!(*data, other);

    Ok(())
}

async fn find_by_name(token: &String, data: &Data) -> Result<()> { 
    let response = reqwest::Client::new()
        .get(format!("{}?name={}", URL, PROJECT_NAME))
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

    let value = &json["items"][0];
    let other: Data = serde_json::from_value(value.clone()).unwrap();
    assert_eq!(*data, other);

    Ok(())
}

async fn update(token: &String, data: &Data) -> Result<Data> {
    let body = json!({
        "name": "Nome Alterado",
        "description": "Descrição alterada...",
    });
    let response = reqwest::Client::new()
        .put(format!("{}/{}", URL, data.project))
        .bearer_auth(token)
        .json(&body)
        .send()
        .await?;
    assert_eq!(response.status(), StatusCode::OK);

    let other = response.json::<Data>().await?;
    assert_eq!(other.project, data.project);
    assert_eq!(other.name, body["name"].as_str().unwrap());
    assert_eq!(other.description.clone().unwrap().as_str(), body["description"].as_str().unwrap());
    assert_eq!(other.time, data.time);
    assert_eq!(other.user, data.user);

    Ok(other)
}

async fn remove(token: &String, data: &Data) -> Result<()> {
    let response = reqwest::Client::new()
        .delete(format!("{}/{}", URL, data.project))
        .bearer_auth(token)
        .send()
        .await?;
    assert_eq!(response.status(), StatusCode::NO_CONTENT); 
    Ok(())
}

async fn not_found(token: &String, data: &Data) -> Result<()> {
    let response = reqwest::Client::new()
        .get(format!("{}/{}", URL, data.project))
        .bearer_auth(token)
        .send()
        .await?;
    assert_eq!(response.status(), StatusCode::NOT_FOUND);    
    Ok(())
}

#[tokio::test]
async fn execute() -> Result<()> {
    let token = tokens::request_token(USERNAME, PASSWORD, Tenant::TENANT_DEFAULT).await?;
    assert!(!token.is_empty());

    list(&token).await?;

    let data = create(&token).await?;

    find_by_id(&token, &data).await?;
    find_by_name(&token, &data).await?;

    let data = update(&token, &data).await?;

    remove(&token, &data).await?;

    not_found(&token, &data).await?;

    Ok(())
}
