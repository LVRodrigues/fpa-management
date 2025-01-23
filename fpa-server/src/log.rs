use axum::http::{Method, Uri};
use chrono::{DateTime, Utc};
use log::info;
use serde::Serialize;

use serde_json::json;
use serde_with::skip_serializing_none;
use uuid::Uuid;

use crate::ctx::Context;

#[skip_serializing_none]
#[derive(Serialize)]
pub struct LogRequest {
    id: Uuid,
    time: DateTime<Utc>,
    user: Option<Uuid>,

    request_path: String,
    request_method: String,
}

pub async fn log_request(uuid: Uuid, method: Method, uri: Uri, context: Option<Context>) {
    let line = LogRequest {
        id: uuid.clone(),
        time: Utc::now(),
        user: context.map(|c| c.id().clone()),

        request_path: uri.to_string(),
        request_method: method.to_string(),
    };
    info!("{}", json!(line));
}
