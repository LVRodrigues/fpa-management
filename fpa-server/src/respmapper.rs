use axum::{response::{Response, IntoResponse}, Json, http::{Uri, Method, method}};
use serde_json::json;
use uuid::Uuid;

use crate::{error::Error, ctx::Context, log};

pub async fn response_mapper(context: Context, uri: Uri, method: Method, response: Response) -> Response {
    println!("==> {:<12} - response_mapper", "MAPPER ");

    let service_error = response.extensions().get::<Error>();
    let client_error = service_error.map(|se| se.client_status_and_error());

    let error_response = client_error.as_ref()
        .map(|(code, error)| {
            let body = json!({
                "error": {
                    "type": error.as_ref(),
                }
            });
            println!("  ==> {body}");
            (*code, Json(body)).into_response()
        });

    let client_error = client_error.unzip().1;
    log::log_request(
        Uuid::new_v4(),
        method,
        uri,
        Some(context),
        service_error,
        client_error
    ).await;

    println!();
    error_response.unwrap_or(response)
}