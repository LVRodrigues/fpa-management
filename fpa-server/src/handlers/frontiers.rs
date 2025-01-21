use std::sync::Arc;

use axum::{
    extract::{Path, Query, State},
    http::{HeaderMap, StatusCode, Uri},
    response::IntoResponse,
    Json,
};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, EntityTrait, ModelTrait, PaginatorTrait, QueryFilter,
    Set,
};
use serde::Deserialize;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::model::prelude::Frontiers;
use crate::{
    ctx::Context,
    error::{Error, ErrorResponse},
    model::{
        frontiers::{self, ActiveModel, Model},
        page::{Page, PageParams},
    },
    state::AppState,
};

/// Search for a set of Modules for a Project.
#[utoipa::path(
    tag = "Frontiers",
    get,
    path = "/api/projects/{project}/frontiers",
    responses(
        (status = OK, description = "Success", body = Page<frontiers::Model>),
        (status = UNAUTHORIZED, description = "User not authorized.", body = ErrorResponse),
        (status = NOT_FOUND, description = "Project not founded.", body = ErrorResponse),
        (status = SERVICE_UNAVAILABLE, description = "FPA Management service unavailable.", body = ErrorResponse)
    ),
    params(
        ("project" = Uuid, description = "Project Unique ID."),
        PageParams,
    ),
    security(("fpa-security" = []))
)]
pub async fn list(
    Path(project): Path<Uuid>,
    Query(params): Query<PageParams>,
    context: Option<Context>,
    state: State<Arc<AppState>>,
) -> Result<impl IntoResponse, Error> {
    println!("==> {:<12} - /{project}/list", "FRONTIERS");

    let mut conditions = Condition::all();
    conditions = conditions.add(frontiers::Column::Project.eq(project));
    if let Some(name) = params.name() {
        conditions = conditions.add(frontiers::Column::Name.contains(&name));
    }

    let ctx = context.unwrap();
    let db = state.connection(ctx.tenant()).await?;
    let paginator = Frontiers::find()
        .filter(conditions)
        .paginate(&db, params.size());

    let items = paginator.fetch_page(params.page() - 1).await?;
    let mut page: Page<Model> = Page::new();
    page.pages = paginator.num_pages().await?;
    page.index = params.page();
    page.size = items.len() as u64;
    page.records = paginator.num_items().await?;
    page.items = items;

    Ok(Json(page))
}

/// Search for a specific Module.
#[utoipa::path(
    tag = "Frontiers",
    get,
    path = "/api/projects/{project}/frontiers/{frontier}",
    responses(
        (status = OK, description = "Success.", body = frontiers::Model),
        (status = UNAUTHORIZED, description = "User not authorized.", body = ErrorResponse),
        (status = NOT_FOUND, description = "Project not founded.", body = ErrorResponse),
        (status = SERVICE_UNAVAILABLE, description = "FPA Management service unavailable.", body = ErrorResponse)
    ),
    params(
        ("project" = Uuid, Path, description = "Project Unique ID."),
        ("frontier" = Uuid, Path, description = "Frontier Unique ID."),
    ),
    security(("fpa-security" = []))    
)]
pub async fn by_id(
    Path((project, frontier)): Path<(Uuid, Uuid)>,
    context: Option<Context>,
    state: State<Arc<AppState>>,
) -> Result<impl IntoResponse, Error> {
    println!("==> {:<12} - /{project}/by_id {frontier}", "MODULES");
    let ctx = context.unwrap();
    let db = state.connection(ctx.tenant()).await?;

    let mut conditions = Condition::all();
    conditions = conditions.add(frontiers::Column::Project.eq(project));
    conditions = conditions.add(frontiers::Column::Frontier.eq(frontier));

    let data = Frontiers::find().filter(conditions).one(&db).await?;
    match data {
        Some(v) => Ok((StatusCode::OK, Json(v))),
        None => return Err(Error::NotFound),
    }
}

/// Frontier's properties.
#[derive(Debug, Deserialize, ToSchema)]
pub struct FrontierParam {
    /// Frontier's name.
    pub name: String,
    /// Description for the Frontier.
    pub description: Option<String>,
}

/// Create a new Frontier for a selected Project.
#[utoipa::path(
    tag = "Frontiers",
    post,
    path = "/api/projects/{project}/frontiers",
    responses(
        (status = CREATED, description = "Success.", body = frontiers::Model, headers(("Location", description = "New Frontier address."))),
        (status = UNAUTHORIZED, description = "User not authorized.", body = ErrorResponse),
        (status = CONFLICT, description = "The name must be unique for the selected project.", body = ErrorResponse),
        (status = SERVICE_UNAVAILABLE, description = "FPA Management service unavailable.", body = ErrorResponse)
    ),
    params(
        ("project" = Uuid, Path, description = "Project Unique ID."),
    ),
    security(("fpa-security" = []))
)]
pub async fn create(
    Path(project): Path<Uuid>,
    context: Option<Context>,
    state: State<Arc<AppState>>,
    Json(params): Json<FrontierParam>,
) -> Result<impl IntoResponse, Error> {
    println!("==> {:<12} - /{project}/create {:?}", "FRONTIERS", params);
    let ctx = context.unwrap();
    let db = state.connection(ctx.tenant()).await?;
    let config = state.configuration();

    let module = frontiers::ActiveModel {
        project: Set(project.clone()),
        tenant: Set(ctx.tenant().clone()),
        frontier: Set(Uuid::now_v7()),
        name: Set(params.name.to_owned()),
        description: Set(params.description.to_owned()),
    };
    let module = match module.insert(&db).await {
        Ok(v) => v,
        Err(e) => {
            match e.sql_err().unwrap() {
                sea_orm::SqlErr::UniqueConstraintViolation(_) => {
                    return Err(Error::ModuleNameDuplicated)
                }
                _ => return Err(Error::ModuleCreate),
            };
        }
    };

    match db.commit().await {
        Ok(it) => it,
        Err(_) => return Err(Error::DatabaseTransaction),
    };

    let location = Uri::builder()
        .scheme(config.scheme.clone())
        .authority(format!(
            "{}:{}",
            config.authority.clone(),
            config.port.clone()
        ))
        .path_and_query(format!(
            "/api/projects/{}/frontiers/{}",
            &module.project, module.frontier
        ))
        .build()
        .unwrap()
        .to_string()
        .parse()
        .unwrap();
    let mut header = HeaderMap::new();
    header.insert("Location", location);

    Ok((StatusCode::CREATED, header, Json(module)))
}

/// Update a existing Frontier.
#[utoipa::path(
    tag = "Frontiers",
    put,
    path = "/api/projects/{project}/frontiers/{frontier}",
    responses(
        (status = OK, description = "Success.", body = frontiers::Model),
        (status = UNAUTHORIZED, description = "User not authorized.", body = ErrorResponse),
        (status = NOT_FOUND, description = "Project or Frontier not founded.", body = ErrorResponse),
        (status = CONFLICT, description = "The name must be unique for the selected project.", body = ErrorResponse),
        (status = SERVICE_UNAVAILABLE, description = "FPA Management service unavailable.", body = ErrorResponse)
    ),
    params(
        ("project" = Uuid, Path, description = "Project Unique ID."),
        ("frontier" = Uuid, Path, description = "Frontier Unique ID."),
    ),
    security(("fpa-security" = []))
)]
pub async fn update(
    Path((project, frontier)): Path<(Uuid, Uuid)>,
    context: Option<Context>,
    state: State<Arc<AppState>>,
    Json(params): Json<FrontierParam>,
) -> Result<impl IntoResponse, Error> {
    println!(
        "==> {:<12} - /{project}/update/{frontier} {:?}",
        "MODULES", params
    );
    let ctx = context.unwrap();
    let db = state.connection(ctx.tenant()).await?;

    let mut conditions = Condition::all();
    conditions = conditions.add(frontiers::Column::Project.eq(project));
    conditions = conditions.add(frontiers::Column::Frontier.eq(frontier));

    let data = Frontiers::find().filter(conditions).one(&db).await?;
    let data = match data {
        Some(v) => v,
        None => return Err(Error::NotFound),
    };

    let mut data: ActiveModel = data.into();
    data.name = Set(params.name);
    data.description = Set(params.description);

    let data: Model = match data.update(&db).await {
        Ok(v) => v,
        Err(e) => {
            match e.sql_err().unwrap() {
                sea_orm::SqlErr::UniqueConstraintViolation(_) => {
                    return Err(Error::ModuleNameDuplicated)
                }
                _ => return Err(Error::ModuleUpdate),
            };
        }
    };

    Ok((StatusCode::OK, Json(data)))
}

/// Remove a existing Frontier.
#[utoipa::path(
    tag = "Frontiers",
    delete,
    path = "/api/projects/{project}/frontiers/{frontier}",
    responses(
        (status = NO_CONTENT, description = "Success."),
        (status = UNAUTHORIZED, description = "User not authorized.", body = ErrorResponse),
        (status = NOT_FOUND, description = "Project or Frontier not founded.", body = ErrorResponse),
        (status = PRECONDITION_FAILED, description = "Frontier has related records.", body = ErrorResponse),
        (status = SERVICE_UNAVAILABLE, description = "FPA Management service unavailable.", body = ErrorResponse),
    ),
    params(
        ("project" = Uuid, Path, description = "Project Unique ID."),
        ("frontier" = Uuid, Path, description = "Frontier Unique ID."),
    ),
    security(("fpa-security" = []))
)]
pub async fn remove(
    Path((project, frontier)): Path<(Uuid, Uuid)>,
    context: Option<Context>,
    state: State<Arc<AppState>>,
) -> Result<impl IntoResponse, Error> {
    println!("==> {:<12} - /{project}/remove/{frontier}", "MODULES");
    let ctx = context.unwrap();
    let db = state.connection(ctx.tenant()).await?;

    let mut conditions = Condition::all();
    conditions = conditions.add(frontiers::Column::Project.eq(project));
    conditions = conditions.add(frontiers::Column::Frontier.eq(frontier));

    let data = Frontiers::find().filter(conditions).one(&db).await?;
    let data = match data {
        Some(v) => v,
        None => return Err(Error::NotFound),
    };

    match data.delete(&db).await {
        Ok(v) => {
            if v.rows_affected != 1 {
                return Err(Error::MultipleRowsAffected);
            }
        }
        Err(_) => return Err(Error::ModuleConstraints),
    };
    match db.commit().await {
        Ok(it) => it,
        Err(_) => return Err(Error::DatabaseTransaction),
    }

    Ok(StatusCode::NO_CONTENT)
}
