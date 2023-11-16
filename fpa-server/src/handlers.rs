use axum::{http::StatusCode, response::IntoResponse};

pub async fn hello() -> impl IntoResponse {
    (StatusCode::OK, "Hello, APF Management")
}