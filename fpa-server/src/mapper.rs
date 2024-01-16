use axum::{response::Response, http::{Uri, Method}};
use uuid::Uuid;

use crate::{ctx::Context, log};


pub async fn response_mapper(context: Option<Context>, uri: Uri, method: Method, response: Response) -> Response {
    println!("==> {:<12} - response_mapper", "MAPPER ");

    log::log_request(
        Uuid::new_v4(),
        method,
        uri,
        context,
    ).await;

    println!();
    response
}