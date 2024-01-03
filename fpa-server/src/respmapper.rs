use axum::{response::{Response, IntoResponse}, Json};
use serde_json::json;

use crate::error::Error;

pub async fn response_mapper(response: Response) -> Response {
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

    // TODO Request unique log...

    println!();
    error_response.unwrap_or(response)
}