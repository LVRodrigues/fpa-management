use std::sync::{Arc, Mutex, OnceLock};

use anyhow::Result;
use reqwest::StatusCode;
use serial_test::serial;
use uuid::Uuid;
use crate::tokens::Tenant;
use serde_json::json;

mod tokens;

const PROJECT_NAME: &str = "Running-Test";

#[derive(Debug)]
struct DataTest {
    created: Option<Uuid>,
}
impl DataTest {
    fn new() -> Self {
        Self { created: None }
    }
}
static TEST: OnceLock<Arc<Mutex<DataTest>>> = OnceLock::new();

#[tokio::test]
#[serial]
async fn list() -> Result<()> {
    let token = tokens::request_token("user", "fpa-pass", Tenant::TENANT_DEFAULT).await?;
    assert!(!token.is_empty());

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
    assert!(!token.is_empty());

    let data = Arc::new(Mutex::new(DataTest::new()));

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

    data.lock().unwrap().created = Some(Uuid::parse_str(json["project"].as_str().unwrap())?);
    TEST.set(data).unwrap();

    Ok(())
}

#[tokio::test]
#[serial]
async fn find_by_id() -> Result<()> {
    let data = match TEST.get() {
        Some(v) => v,
        None => panic!("Run create() test first.")
    };
    let data = data.lock().unwrap();
    assert!(data.created.is_some());

    let token = tokens::request_token("user", "fpa-pass", Tenant::TENANT_DEFAULT).await?;
    assert!(!token.is_empty());

    let response = reqwest::Client::new()
        .get(format!("http://localhost:5000/api/projects/{}", data.created.unwrap()))
        .bearer_auth(token)
        .send()
        .await?;
    assert_eq!(response.status(), StatusCode::OK);

    let json = response.json::<serde_json::Value>().await?;
    assert_eq!(json["project"].as_str().unwrap(), data.created.unwrap().to_string().as_str());
    assert_eq!(json["name"].as_str().unwrap(), PROJECT_NAME);

    Ok(())
}