use ::log::trace;
use axum::{
    http::{Method, Uri},
    response::Response,
};
use uuid::Uuid;

use crate::{ctx::Context, log};

pub async fn response_mapper(
    context: Option<Context>,
    uri: Uri,
    method: Method,
    response: Response,
) -> Response {
    trace!("Registering the request.");

    log::log_request(Uuid::now_v7(), method, uri, context).await;

    response
}
