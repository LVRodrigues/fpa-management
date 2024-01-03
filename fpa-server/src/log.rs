use crate::Error;
use serde::Serialize;
use serde_json::{json, Value};
use serde_with::skip_serializing_none;
use uuid::Uuid;

#[skip_serializing_none]
#[derive(Serialize)]
pub struct LogRequest {
    id: Uuid,
    time: String,       // timestamp iso8601
    user: Option<Uuid>,

    request_path: String,
    request_method: String,

    client_error_type: Option<String>,
    error_type: Option<String>,
    error_data: Option<String>,
}