use std::sync::{Arc, Mutex, OnceLock};

use anyhow::Result;
use reqwest::StatusCode;
use uuid::Uuid;
use crate::tokens::Tenant;
use serde_json::json;

mod tokens;

const PROJECT_NAME: &str = "Running-Test";

#[derive(Debug)]
struct Data {
    project: Option<Uuid>,
}
impl Data {
    fn new() -> Self {
        Self { project: None }
    }
}
static TEST: OnceLock<Arc<Mutex<Data>>> = OnceLock::new();

async fn list(token: &String) -> Result<()> {
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

async fn create(token: &String) -> Result<()> {
    let data = Arc::new(Mutex::new(Data::new()));

    let response = reqwest::Client::new()
        .post(format!("http://localhost:5000/api/projects?name={}", PROJECT_NAME))
        .bearer_auth(token)
        .send()
        .await?;
    assert!(response.status() == StatusCode::CREATED);
    assert!(response.headers().get("location").is_some());

    let json = response.json::<serde_json::Value>().await?;
    assert!(json.get("project").is_some());
    assert_eq!(*json.get("name").unwrap(), PROJECT_NAME);
    assert!(json.get("time").is_some());
    assert!(json.get("user").is_some());

    data.lock().unwrap().project = Some(Uuid::parse_str(json["project"].as_str().unwrap())?);
    TEST.set(data).unwrap();
    assert!(TEST.get().unwrap().lock().unwrap().project.is_some());

    Ok(())
}

async fn find_by_id(token: &String) -> Result<()> { 
    let data = match TEST.get() {
        Some(v) => v,
        None => panic!("Run create() test first.")
    };
    let data = data.lock().unwrap();
    assert!(data.project.is_some());

    let response = reqwest::Client::new()
    .get(format!("http://localhost:5000/api/projects/{}", data.project.unwrap()))
    .bearer_auth(token)
    .send()
    .await?;
    assert_eq!(response.status(), StatusCode::OK);

    let json = response.json::<serde_json::Value>().await?;
    assert_eq!(json["project"].as_str().unwrap(), data.project.unwrap().to_string().as_str());
    assert_eq!(json["name"].as_str().unwrap(), PROJECT_NAME);

    Ok(())
}


#[tokio::test]
async fn execute() -> Result<()> {
    let token = tokens::request_token("user", "fpa-pass", Tenant::TENANT_DEFAULT).await?;
    assert!(!token.is_empty());

    list(&token).await?;
    create(&token).await?;
    find_by_id(&token).await?;

    Ok(())
}