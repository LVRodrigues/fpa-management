
use std::sync::Arc;

use axum::{extract::{Path, State}, response::IntoResponse, Json};
use sea_orm::{EntityTrait, ModelTrait};
use uuid::Uuid;

use crate::{ctx::Context, error::Error, model::{empiricals::{self, Model}, page::Page, prelude::{Empiricals, Projects}, projects}, state::AppState};

/// Search for a set of Empirical's Factor for a Project.
#[utoipa::path(
    tag = "Empiricals",
    get,
    path = "/api/projects/{id}/empiricals",
    responses(
        (status = OK, description = "Success", body = Empiricals),
        (status = UNAUTHORIZED, description = "User not authorized.", body = Error),
        (status = NOT_FOUND, description = "Project not founded.", body = Error),
        (status = SERVICE_UNAVAILABLE, description = "FPA Management service unavailable.", body = Error)
    ),
    params(
        ("id" = Uuid, description = "Project Unique ID.")
    ),
    security(("fpa-security" = []))
)]
pub async fn list(Path(id): Path<Uuid>, context: Option<Context>, state: State<Arc<AppState>>) -> Result<impl IntoResponse, Error> {
    println!("==> {:<12} - /{id}/empiricals (Params: {:?})", "EMPIRICALS", id);
    let ctx = context.unwrap();
    let db = state.connection(ctx.tenant()).await?;

    let project = match Projects::find_by_id(id).one(&db).await.unwrap() {
        Some(v) => v,
        None => return Err(Error::NotFound),
    };

    let items = project.find_related(Empiricals).all(&db).await?;
    let mut page: Page<Model> = Page::new();
    page.pages      = 1;
    page.index      = 1;
    page.size       = items.len() as u64;
    page.records    = items.len() as u64;
    page.items      = items;

    Ok(Json(page))
}