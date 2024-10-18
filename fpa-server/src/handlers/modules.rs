use std::sync::Arc;

use sea_orm::{ActiveModelTrait, ColumnTrait, Condition, EntityTrait, ModelTrait, Paginator, PaginatorTrait, QueryFilter, Set};
use serde::Deserialize;
use utoipa::ToSchema;
use uuid::Uuid;
use axum::{extract::{Path, Query, State}, http::StatusCode, response::IntoResponse, Json};

use crate::{ctx::Context, error::{Error, ErrorResponse}, model::{modules::{self, ActiveModel, Model}, page::{Page, PageParams}, prelude::{Modules, Projects}}, state::AppState};

/// Search for a set of Modules for a Project.
#[utoipa::path(
    tag = "Modules",
    get,
    path = "/api/projects/{id}/modules",
    responses(
        (status = OK, description = "Success", body = Page<modules::Model>),
        (status = UNAUTHORIZED, description = "User not authorized.", body = ErrorResponse),
        (status = NOT_FOUND, description = "Project not founded.", body = ErrorResponse),
        (status = SERVICE_UNAVAILABLE, description = "FPA Management service unavailable.", body = ErrorResponse)
    ),
    params(
        ("id" = Uuid, description = "Project Unique ID."),
        PageParams,
    ),
    security(("fpa-security" = []))
)]
pub async fn list(Path(id): Path<Uuid>, Query(params): Query<PageParams>, context: Option<Context>, state: State<Arc<AppState>>) -> Result<impl IntoResponse, Error> {
    println!("==> {:<12} - /{id}/list", "MODULES");

    let mut conditions = Condition::all();
    conditions = conditions.add(modules::Column::Project.eq(id));
    if let Some(name) = params.name() {
        conditions = conditions.add(modules::Column::Name.contains(&name));
    }

    let ctx = context.unwrap();
    let db = state.connection(ctx.tenant()).await?;
    let paginator = Modules::find()
        .filter(conditions)
        .paginate(&db, params.size());

    let items = paginator.fetch_page(params.page() - 1).await?;
    let mut page: Page<Model> = Page::new();
    page.pages      = 1;
    page.index      = 1;
    page.size       = items.len() as u64;
    page.records    = items.len() as u64;
    page.items      = items;

    Ok(Json(page))
}
