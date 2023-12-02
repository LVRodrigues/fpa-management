use crate::error::{Error, Result};

use axum::{http::{Request, header}, middleware::Next, response::Response};

const BEARER: &str = "Bearer ";

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
    
    Ok(next.run(request).await)
}