use std::sync::Arc;

use axum::{extract::State, response::IntoResponse, Json};
use sea_orm::EntityTrait;
use crate::{ctx::Context, error::Error, model::prelude::Projects, state::AppState};

#[utoipa::path(
    tag = "Projects",
    get,
    path = "/api/projects",
    responses(
        (status = OK, description = "Sucess.", body = Projects),
        (status = SERVICE_UNAVAILABLE, description = "FPA Management service unavailable.")
    ),
    security(("fpa-security" = []))
)]
pub async fn list(context: Option<Context>, State(state): State<Arc<AppState>>) -> Result<impl IntoResponse, Error> {
    println!("==> {:<12} - /list", "PROJECTS");

    let db = state.connection(context.unwrap().tenant()).await?;
    let items = Projects::find().all(&db).await?;
    Ok(Json(items))
}