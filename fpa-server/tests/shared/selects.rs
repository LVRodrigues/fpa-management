use uuid::Uuid;

use anyhow::Result;
use reqwest::StatusCode;

use crate::shared::URL;

pub async fn project(token: &String) -> Result<Uuid> {
    let response = reqwest::Client::new()
        .get(URL)
        .bearer_auth(token)
        .send()
        .await?;
    assert_eq!(response.status(), StatusCode::OK);
    
    let json = response.json::<serde_json::Value>().await?;
    let result = json["items"].as_array().unwrap()[0].clone();
    let result = Uuid::parse_str(result["project"].as_str().unwrap()).unwrap();

    Ok(result)
}