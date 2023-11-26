use error::{Error, Result};

use crate::error;

use axum::{http::{Request, header}, middleware::Next, response::Response};

pub async fn require<B>(request: Request<B>, next: Next<B>) -> Result<Response> {
    let token = request.headers()
        .get(header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok())
        .and_then(|value| {
            if value.starts_with("Bearer ") {
                Some(value[7..].to_owned())
            } else {
                None
            }
        });
    println!("==> {:<12} Token: {:?}", "AUTH", token);
    Ok(next.run(request).await)
}