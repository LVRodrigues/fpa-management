use anyhow::Result;
use reqwest::StatusCode;
use uuid::Uuid;
use crate::tokens::Tenant;
use serde_json::json;

mod tokens;

const USERNAME: &str = "user";
const PASSWORD: &str = "fpa-pass";

const PROJECT_NAME: &str = "Running-Test";

#[derive(Debug)]
struct Data {
    project: Uuid,
}
impl Data {
    fn new(id: Uuid) -> Self {
        Self { project: id }
    }
}

async fn list(token: &String) -> Result<()> {
    let response = reqwest::Client::new()
        .get("http://localhost:5000/api/projects")
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
    let response = reqwest::Client::new()
        .post(format!("http://localhost:5000/api/projects?name={}", PROJECT_NAME))
        .bearer_auth(token)
        .send()
        .await?;
    assert!(response.status() == StatusCode::CREATED);
    assert!(response.headers().get("location").is_some());

    let json = response.json::<serde_json::Value>().await?;
    assert!(json.get("project").is_some());
    assert_eq!(json["name"].as_str().unwrap(), PROJECT_NAME);
    assert!(json.get("time").is_some());
    assert!(json.get("user").is_some());

    let id = Uuid::parse_str(json["project"].as_str().unwrap())?;
    let data = Data::new(id);
    assert!(!data.project.is_nil());

    Ok(data)
}

async fn find_by_id(token: &String, id: &Uuid) -> Result<()> { 
    let response = reqwest::Client::new()
        .get(format!("http://localhost:5000/api/projects/{}", id))
        .bearer_auth(token)
        .send()
        .await?;
    assert_eq!(response.status(), StatusCode::OK);

    let json = response.json::<serde_json::Value>().await?;
    assert_eq!(json["project"].as_str().unwrap(), id.to_string().as_str());
    assert_eq!(json["name"].as_str().unwrap(), PROJECT_NAME);

    Ok(())
}

async fn find_by_name(token: &String) -> Result<()> { 
    let response = reqwest::Client::new()
        .get(format!("http://localhost:5000/api/projects?name={}", PROJECT_NAME))
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
    assert_eq!(json["items"][0]["name"].as_str().unwrap(), PROJECT_NAME);
    Ok(())
}

#[tokio::test]
async fn execute() -> Result<()> {
    let token = tokens::request_token(USERNAME, PASSWORD, Tenant::TENANT_DEFAULT).await?;
    assert!(!token.is_empty());

    list(&token).await?;

    let data = create(&token).await?;

    find_by_id(&token, &data.project).await?;
    find_by_name(&token).await?;

    Ok(())
}