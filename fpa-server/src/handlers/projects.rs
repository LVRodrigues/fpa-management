use std::sync::Arc;

use axum::{extract::{Path, Query, State}, http::{HeaderMap, StatusCode, Uri}, response::IntoResponse, Json};
use chrono::Utc;
use sea_orm::{ActiveModelTrait, ColumnTrait, Condition, EntityTrait, PaginatorTrait, QueryFilter, Set};
use serde_derive::Deserialize;
use utoipa::IntoParams;
use uuid::Uuid;
use crate::{ctx::Context, error::Error, model::{page::{Page, PageParams}, prelude::Projects, projects}, state::AppState};

/// Select a set of projects.
#[utoipa::path(
    tag = "Projects",
    get,
    path = "/api/projects",
    responses(
        (status = OK, description = "Sucess.", body = Projects),
        (status = UNAUTHORIZED, description = "User not authorized.", body = Error),
        (status = SERVICE_UNAVAILABLE, description = "FPA Management service unavailable.", body = Error)
    ),
    params(PageParams),
    security(("fpa-security" = []))
)]
pub async fn list(params: Query<PageParams>, context: Option<Context>, state: State<Arc<AppState>>) -> Result<impl IntoResponse, Error> {
    println!("==> {:<12} - /list (Params: {:?})", "PROJECTS", params);

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
    let mut page: Page<projects::Model> = Page::new();
    page.pages      = paginator.num_pages().await?;
    page.index      = params.page();
    page.size       = items.len() as u64;
    page.records    = paginator.num_items().await?;
    page.items      = items;

    Ok(Json(page))
}

/// Select a specific project.
#[utoipa::path(
    tag = "Projects",
    get,
    path = "/api/projects/{id}",
    responses(
        (status = OK, description = "Sucess.", body = Project),
        (status = UNAUTHORIZED, description = "User not authorized.", body = Error),
        (status = NOT_FOUND, description = "Project not founded.", body = Error),
        (status = SERVICE_UNAVAILABLE, description = "FPA Management service unavailable.", body = Error)
    ),
    params(
        ("id" = Uuid, Path, description = "Project Unique ID.")
    ),
    security(("fpa-security" = []))
)]
pub async fn by_id(Path(id): Path<Uuid>, context: Option<Context>, state: State<Arc<AppState>>) -> Result<impl IntoResponse, Error> {
    println!("==> {:<12} - /byId (id: {:?})", "PROJECTS", id);
    let ctx = context.unwrap();
    let db = state.connection(ctx.tenant()).await?;

    let project: Option<projects::Model> = Projects::find_by_id(id).one(&db).await?;
    match project {
        Some(v) => Ok((StatusCode::OK, Json(v))),
        None => return Err(Error::NotFound),
    }
}

/// Project create params.
#[derive(Debug, Deserialize, IntoParams)]
pub struct ProjectCreateParam {
    /// New Project' name.
    pub name: String,
}

/// Create a new Project.
#[utoipa::path(
    tag = "Projects",
    post,
    path = "/api/projects",
    responses(
        (status = CREATED, description = "Sucess.", body = Project, headers(("Location", description = "New project address."))),
        (status = UNAUTHORIZED, description = "User not authorized.", body = Error),
        (status = SERVICE_UNAVAILABLE, description = "FPA Management service unavailable.", body = Error)
    ),
    params(ProjectCreateParam),
    security(("fpa-security" = []))
)]
pub async fn create(param: Query<ProjectCreateParam>, context: Option<Context>, state: State<Arc<AppState>>) -> Result<impl IntoResponse, Error> {
    println!("==> {:<12} - /create (Name: {:?})", "PROJECTS", param);
    let ctx = context.unwrap();
    let db = state.connection(ctx.tenant()).await?;

    let project = projects::ActiveModel {
        project: Set(Uuid::new_v4()),
        tenant: Set(ctx.tenant().clone()),
        user: Set(ctx.id().clone()),
        time: Set(Utc::now().into()),
        name: Set(param.name.to_string()),
    };
    let project: projects::Model = match project.insert(&db).await {
        Ok(v) => {
            println!(" -> New Project: {:?}", v);
            v
        },
        Err(_) => return Err(Error::ProjectCreate),
    };
    match db.commit().await {
        Ok(it) => it,
        Err(_) => return Err(Error::DatabaseTransaction),
    };

    let config = state.configuration();
    let location = Uri::builder()
        .scheme(config.scheme.clone())
        .authority(format!("{}:{}", config.authority.clone(), config.port.clone()))
        .path_and_query(format!("/api/projects/{}", &project.project))
        .build()
        .unwrap()
        .to_string()
        .parse()
        .unwrap();
    let mut header = HeaderMap::new();
    header.insert("Location", location);

    Ok((StatusCode::CREATED, header, Json(project)))
}