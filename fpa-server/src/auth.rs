use error::{Error, Result};

use crate::error;

use axum::{http::Request, middleware::Next, response::Response};

pub async fn require<B>(request: Request<B>, next: Next<B>) -> Result<Response> {
    println!("==> {:<12} - [token?]", "AUTH");
    Ok(next.run(request).await)
}