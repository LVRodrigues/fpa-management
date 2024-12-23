use std::sync::Arc;

use axum::{extract::{Path, Query, State}, response::IntoResponse};
use uuid::Uuid;

use crate::{ctx::Context, error::{Error, ErrorResponse}, model::{functions, page::{Page, PageParams}}, state::AppState};

#[utoipa::path(
    tag = "Functions",
    get,
    path = "/api/projects/{project}/modules/{module}/functions",
    responses(
        (status = OK, description = "Success", body = Page<functions::Model>),
        (status = UNAUTHORIZED, description = "User not authorized.", body = ErrorResponse),
        (status = NOT_FOUND, description = "Project not founded.", body = ErrorResponse),
        (status = SERVICE_UNAVAILABLE, description = "FPA Management service unavailable.", body = ErrorResponse)
    ),
    params(
        ("project" = Uuid, description = "Project Unique ID."),
        ("module" = Uuid, Path, description = "Module Unique ID."),
        PageParams,
    ),
    security(("fpa-security" = []))
)]
pub async fn list(Path((project, module)): Path<(Uuid, Uuid)>, params: Query<PageParams>, context: Option<Context>, state: State<Arc<AppState>>) -> Result<impl IntoResponse, Error> {
    println!("==> {:<12} - /list (Params: {:?})", "FUNCTIONS", params);

    Ok(())
}