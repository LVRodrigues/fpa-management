use std::sync::Arc;

use sea_orm::{ColumnTrait, Condition, EntityTrait, PaginatorTrait, QueryFilter};
use uuid::Uuid;
use axum::{extract::{Path, Query, State}, http::StatusCode, response::IntoResponse, Json};

use crate::{ctx::Context, error::{Error, ErrorResponse}, model::{modules::{self, Model}, page::{Page, PageParams}}, state::AppState};
use crate::model::prelude::Modules;

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

/// Search for a specific Module.
#[utoipa::path(
    tag = "Modules",
    get,
    path = "/api/projects/{id}/modules/{module}",
    responses(
        (status = OK, description = "Success.", body = modules::Model),
        (status = UNAUTHORIZED, description = "User not authorized.", body = ErrorResponse),
        (status = NOT_FOUND, description = "Project not founded.", body = ErrorResponse),
        (status = SERVICE_UNAVAILABLE, description = "FPA Management service unavailable.", body = ErrorResponse)
    ),
    params(
        ("id" = Uuid, Path, description = "Project Unique ID."),
        ("module" = Uuid, Path, description = "Module Unique ID."),
    ),
    security(("fpa-security" = []))    
)]
pub async fn by_id(Path(id): Path<Uuid>, Path(module): Path<Uuid>, context: Option<Context>, state: State<Arc<AppState>>) -> Result<impl IntoResponse, Error> {
    println!("==> {:<12} - /{id}/by_id {module}", "MODULES");
    let ctx = context.unwrap();
    let db = state.connection(ctx.tenant()).await?;

    let mut conditions = Condition::all();
    conditions = conditions.add(modules::Column::Project.eq(id));
    conditions = conditions.add(modules::Column::Module.eq(module));

    let data = Modules::find().filter(conditions).one(&db).await?;
    match data {
        Some(v) => Ok((StatusCode::OK, Json(v))),
        None => return Err(Error::NotFound),
    }
}
