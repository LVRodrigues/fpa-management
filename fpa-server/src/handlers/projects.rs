use std::sync::Arc;

use crate::{
    ctx::Context,
    error::{Error, ErrorResponse},
    model::{
        page::{Page, PageParams},
        prelude::*,
        projects::{self, ActiveModel, Model},
    },
    state::AppState,
};
use axum::{
    extract::{Path, Query, State},
    http::{HeaderMap, StatusCode, Uri},
    response::IntoResponse,
    Json,
};
use chrono::Utc;
use log::{debug, trace};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, EntityTrait, ModelTrait, PaginatorTrait, QueryFilter,
    Set,
};
use serde_derive::Deserialize;
use serde_json::json;
use utoipa::ToSchema;
use uuid::Uuid;

/// Search for a set of Projects.
#[utoipa::path(
    tag = "Projects",
    get,
    path = "/api/projects",
    responses(
        (status = OK, description = "Success.", body = Page<projects::Model>),
        (status = UNAUTHORIZED, description = "User not authorized.", body = ErrorResponse),
        (status = SERVICE_UNAVAILABLE, description = "FPA Management service unavailable.", body = ErrorResponse)
    ),
    params(PageParams),
    security(("fpa-security" = []))
)]
pub async fn list(
    params: Query<PageParams>,
    context: Option<Context>,
    state: State<Arc<AppState>>,
) -> Result<impl IntoResponse, Error> {
    debug!("List all projects (params: {:?})", params);

    let mut conditions = Condition::all();
    if let Some(name) = params.name() {
        conditions = conditions.add(projects::Column::Name.contains(&name));
    }

    let ctx = context.unwrap();
    let db = state.connection(ctx.tenant()).await?;
    let paginator = Projects::find()
        .filter(conditions)
        .paginate(&db, params.size());

    let items = paginator.fetch_page(params.page() - 1).await?;
    let mut page: Page<Model> = Page::new();
    page.pages = paginator.num_pages().await?;
    page.index = params.page();
    page.size = items.len() as u64;
    page.records = paginator.num_items().await?;
    page.items = items;

    trace!("::: {:?}", json!(page));
    Ok(Json(page))
}

/// Select a specific Project.
#[utoipa::path(
    tag = "Projects",
    get,
    path = "/api/projects/{project}",
    responses(
        (status = OK, description = "Success.", body = projects::Model),
        (status = UNAUTHORIZED, description = "User not authorized.", body = ErrorResponse),
        (status = NOT_FOUND, description = "Project not founded.", body = ErrorResponse),
        (status = SERVICE_UNAVAILABLE, description = "FPA Management service unavailable.", body = ErrorResponse)
    ),
    params(
        ("project" = Uuid, Path, description = "Project Unique ID.")
    ),
    security(("fpa-security" = []))
)]
pub async fn by_id(
    Path(project): Path<Uuid>,
    context: Option<Context>,
    state: State<Arc<AppState>>,
) -> Result<impl IntoResponse, Error> {
    debug!("Select a specific project (project: {:?})", project);

    let ctx = context.unwrap();
    let db = state.connection(ctx.tenant()).await?;

    let data = match Projects::find_by_id(project).one(&db).await? {
        Some(v) => v,
        None => return Err(Error::NotFound),
    };

    trace!("::: {:?}", json!(data));
    Ok(Json(data))
}

/// Project's properties.
#[derive(Debug, Deserialize, ToSchema)]
pub struct ProjectParam {
    /// Project's name.
    pub name: String,
    /// Project's description.
    pub description: Option<String>,
}

/// Create a new Project.
#[utoipa::path(
    tag = "Projects",
    post,
    path = "/api/projects",
    responses(
        (status = CREATED, description = "Success.", body = projects::Model, headers(("Location", description = "New project address."))),
        (status = UNAUTHORIZED, description = "User not authorized.", body = ErrorResponse),
        (status = CONFLICT, description = "The project name must be unique.", body = ErrorResponse),
        (status = SERVICE_UNAVAILABLE, description = "FPA Management service unavailable.", body = ErrorResponse)
    ),
    security(("fpa-security" = []))
)]
pub async fn create(
    context: Option<Context>,
    state: State<Arc<AppState>>,
    Json(params): Json<ProjectParam>,
) -> Result<impl IntoResponse, Error> {
    debug!("Create a new project ({:?})", params);

    let ctx = context.unwrap();
    let db = state.connection(ctx.tenant()).await?;
    let config = state.configuration();

    let project = projects::ActiveModel {
        project: Set(Uuid::now_v7()),
        tenant: Set(ctx.tenant().clone()),
        user: Set(ctx.id().clone()),
        time: Set(Utc::now().into()),
        name: Set(params.name.to_owned()),
        description: Set(params.description.to_owned()),
    };
    let project: projects::Model = match project.insert(&db).await {
        Ok(v) => v,
        Err(e) => {
            match e.sql_err().unwrap() {
                sea_orm::SqlErr::UniqueConstraintViolation(_) => {
                    return Err(Error::ProjectNameDuplicated)
                }
                _ => return Err(Error::ProjectCreate),
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
        .path_and_query(format!("/api/projects/{}", &project.project))
        .build()
        .unwrap()
        .to_string()
        .parse()
        .unwrap();
    trace!("::: {:?}", location);

    let mut header = HeaderMap::new();
    header.insert("Location", location);

    trace!("::: {:?}", json!(project));
    Ok((StatusCode::CREATED, header, Json(project)))
}

/// Update a existing Project.
#[utoipa::path(
    tag = "Projects",
    put,
    path = "/api/projects/{project}",
    responses(
        (status = OK, description = "Success.", body = projects::Model),
        (status = UNAUTHORIZED, description = "User not authorized.", body = ErrorResponse),
        (status = NOT_FOUND, description = "Project not founded.", body = ErrorResponse),
        (status = CONFLICT, description = "The project name must be unique.", body = ErrorResponse),
        (status = SERVICE_UNAVAILABLE, description = "FPA Management service unavailable.", body = ErrorResponse)
    ),
    params(
        ("project" = Uuid, Path, description = "Project Unique ID.")
    ),
    security(("fpa-security" = []))
)]
pub async fn update(
    Path(project): Path<Uuid>,
    context: Option<Context>,
    state: State<Arc<AppState>>,
    Json(params): Json<ProjectParam>,
) -> Result<impl IntoResponse, Error> {
    debug!(
        "Update a existing project (project: {:?} - params: {:?})",
        project, params
    );

    let ctx = context.unwrap();
    let db = state.connection(ctx.tenant()).await?;

    let data: Option<Model> = Projects::find_by_id(project).one(&db).await?;
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
                    return Err(Error::ProjectNameDuplicated)
                }
                _ => return Err(Error::ProjectUpdate),
            };
        }
    };

    match db.commit().await {
        Ok(it) => it,
        Err(_) => return Err(Error::DatabaseTransaction),
    };

    trace!("::: {:?}", json!(data));
    Ok((StatusCode::OK, Json(data)))
}

/// Remove a existing Project.
#[utoipa::path(
    tag = "Projects",
    delete,
    path = "/api/projects/{project}",
    responses(
        (status = NO_CONTENT, description = "Success."),
        (status = UNAUTHORIZED, description = "User not authorized.", body = ErrorResponse),
        (status = NOT_FOUND, description = "Project not founded.", body = ErrorResponse),
        (status = PRECONDITION_FAILED, description = "Project has related records.", body = ErrorResponse),
        (status = SERVICE_UNAVAILABLE, description = "FPA Management service unavailable.", body = ErrorResponse),
    ),
    params(
        ("project" = Uuid, Path, description = "Project Unique ID.")
    ),
    security(("fpa-security" = []))
)]
pub async fn remove(
    Path(project): Path<Uuid>,
    context: Option<Context>,
    state: State<Arc<AppState>>,
) -> Result<impl IntoResponse, Error> {
    debug!("Remove a existing project (project: {:?})", project);

    let ctx = context.unwrap();
    let db = state.connection(ctx.tenant()).await?;

    let data: Option<Model> = Projects::find_by_id(project).one(&db).await?;
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
        Err(_) => return Err(Error::ProjectConstraints),
    };
    match db.commit().await {
        Ok(it) => it,
        Err(_) => return Err(Error::DatabaseTransaction),
    };

    trace!("::: Project {} removed.", project);
    Ok(StatusCode::NO_CONTENT)
}
