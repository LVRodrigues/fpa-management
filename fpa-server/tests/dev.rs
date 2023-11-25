#![allow(unused)]

use anyhow::Result;

#[tokio::test]
async fn dev() -> Result<()> {
    let client = httpc_test::new_client("http://localhost:5000")?;

    client.do_get("/api/hello").await?.print().await?;

    Ok(())
}