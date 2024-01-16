use anyhow::Result;
use reqwest::StatusCode;

mod tokens;

#[tokio::test]
async fn unauth() -> Result<()> {
    let token = tokens::request_token().await?;

    let response = reqwest::Client::new()
        .get("http://localhost:5000/api/health")
        .bearer_auth(token)
        .send()
        .await?;
    println!("Status: {:?}", response.status());
    assert!(response.status() == StatusCode::NO_CONTENT);

    Ok(())
}