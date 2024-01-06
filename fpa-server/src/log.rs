use std::time::{SystemTime, UNIX_EPOCH};

use axum::http::{Method, Uri};
use serde::Serialize;

use serde_json::json;
use serde_with::skip_serializing_none;
use uuid::Uuid;

use crate::{ctx::Context, error::{Error, ClientError}};

#[skip_serializing_none]
#[derive(Serialize)]
pub struct LogRequest {
    id: Uuid,
    time: String, // timestamp iso8601
    user: Option<Uuid>,

    request_path: String,
    request_method: String,

    client_error_type: Option<String>,
    error_type: Option<String>,
    error_data: Option<String>,
}

pub async fn log_request(
    uuid: Uuid,
    method: Method,
    uri: Uri,
    context: Option<Context>,
    service_error: Option<&Error>,
    client_error: Option<ClientError>,
) {
	let timestamp = SystemTime::now()
		.duration_since(UNIX_EPOCH)
		.unwrap()
		.as_millis();

	let error_type = service_error.map(|se| se.to_string());
	let error_data = serde_json::to_value(service_error)
		.ok()
		.and_then(|mut v| v.get_mut("data").map(|v| v.take().to_string()));

	let line = LogRequest {
		id: uuid.clone(),
		time: timestamp.to_string(),
        user: context.map(|c| c.id().clone()),

        request_path: uri.to_string(),
        request_method: method.to_string(),

        client_error_type: client_error.map(|e| e.as_ref().to_string()),
		error_type,
		error_data,
	};

	println!("   ->> log_request: \n{}", json!(line));
}
