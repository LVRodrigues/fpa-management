mod shared;

use anyhow::{Ok, Result};
use reqwest::StatusCode;
use serde_json::json;
use shared::{
    selects,
    tokens::{self, Tenant},
    PASSWORD, URL, USERNAME,
};
use uuid::Uuid;

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

async fn list_by_name(token: &String, project: &Uuid, module: &Uuid) -> Result<()> {
    let response = reqwest::Client::new()
        .get(format!(
            "{}/{}/modules/{}/functions?name=ALI",
            URL, project, module
        ))
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

    let items = json["items"].as_array().unwrap();
    assert!(items[0].get("ALI").is_some());
    let value = items[0].get("ALI").unwrap();
    assert!(value["name"].as_str().unwrap().contains("ALI"));

    assert!(value["rlrs"].is_array());
    assert_eq!(value["rlrs"].as_array().unwrap().len(), 1);

    let ders = value["rlrs"].as_array().unwrap();
    assert_eq!(ders[0]["ders"].as_array().unwrap().len(), 5);

    Ok(())
}

async fn list_by_type(token: &String, project: &Uuid, module: &Uuid) -> Result<()> {
    let response = reqwest::Client::new()
        .get(format!(
            "{}/{}/modules/{}/functions?type=CE",
            URL, project, module
        ))
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

    let items = json["items"].as_array().unwrap();
    assert!(items[0].get("CE").is_some());
    let value = items[0].get("CE").unwrap();
    assert!(value["name"].as_str().unwrap().contains("CE"));

    assert!(value["alrs"].is_array());
    assert_eq!(value["alrs"].as_array().unwrap().len(), 1);

    let alrs = value["alrs"].as_array().unwrap();
    assert!(alrs[0].get("AIE").is_some());

    Ok(())
}

#[tokio::test]
async fn execute() -> Result<()> {
    let token = tokens::request_token(USERNAME, PASSWORD, Tenant::TENANT_DEFAULT).await?;
    assert!(!token.is_empty());

    let project = selects::project(&token).await?;
    let module = selects::module(&token, &project).await?;

    list(&token, &project, &module).await?;

    list_by_name(&token, &project, &module).await?;

    list_by_type(&token, &project, &module).await?;

    Ok(())
}
