use crate::error::{Error, Result};

use axum::{http::{Request, header}, middleware::Next, response::Response};

const BEARER: &str = "Bearer ";
const JWKS_TENANT_01: &str = "http://localhost:8080/realms/tenant-01/protocol/openid-connect/certs";

async fn request_jwks() -> Result<String> {
    let jwks = reqwest::Client::new()
        .get(JWKS_TENANT_01)
        .send()
        .await?
        .text()
        .await?;

    Ok(jwks)
}

pub async fn require<B>(request: Request<B>, next: Next<B>) -> Result<Response> {
    let token = request.headers()
        .get(header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok())
        .and_then(|value| {
            if value.starts_with(BEARER) {
                Some(value[BEARER.len()..].to_owned())
            } else {
                None
            }
        });
    let token = match token {
        Some(v) => v,
        None => return Err(Error::Unauthorized),
    };
    println!("Token: {:?}", token);

    let jwks = request_jwks().await?;
    println!("JWKS: {:?}", jwks);
    
    Ok(next.run(request).await)
}