#![allow(unused)]

use anyhow::Result;

#[tokio::test]
async fn dev() -> Result<()> {
    let response = reqwest::Client::new()
        .get("http://localhost:5000/api/hello")
        .bearer_auth("FAILED")
        .send()
        .await?;

    println!("{:?}", response);

    Ok(())
}