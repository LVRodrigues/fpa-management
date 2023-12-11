use axum::{http::StatusCode, response::IntoResponse, Router, routing::get, middleware};
use utoipa::OpenApi;

use crate::error::{Error, Result};
use crate::auth::{self, Claims};

#[derive(OpenApi)]
#[openapi(paths(hello))]
pub struct ApiDoc;

pub fn router() -> Router {
    Router::new()
        .nest("/api", Router::new().to_owned()
            .route("/hello", get(hello))
            .route_layer(middleware::from_fn(auth::require))
    )
}

#[utoipa::path(
    get,
    path = "/api/hello",
    responses(
        (status = 200, description = "Send a salute from FPA Management.")
    )
)]
pub async fn hello(claims: Claims) -> impl IntoResponse {
    println!("==> {:<12} [/hello]", "HANDLER");
    println!("{:?}", claims);
    (StatusCode::OK, "Hello, APF Management!")
}