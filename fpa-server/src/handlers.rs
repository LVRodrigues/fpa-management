use axum::{http::StatusCode, response::IntoResponse, Router, routing::get};
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(paths(hello))]
pub struct ApiDoc;

pub fn router() -> Router {
    Router::new()
        .nest("/api", Router::new().to_owned()
            .route("/hello", get(hello))
    )
}

#[utoipa::path(
    get,
    path = "/api/hello",
    responses(
        (status = 200, description = "Send a salute from FPA Management.")
    )
)]
pub async fn hello() -> impl IntoResponse {
    println!("==> {:12} [/hello]", "HANDLER");
    (StatusCode::OK, "Hello, APF Management!")
}