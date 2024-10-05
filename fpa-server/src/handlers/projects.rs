use std::sync::Arc;

use axum::{extract::{Path, Query, State}, http::{HeaderMap, StatusCode, Uri}, response::IntoResponse, Json};
use chrono::Utc;
use sea_orm::{ActiveModelTrait, ColumnTrait, Condition, EntityTrait, ModelTrait, PaginatorTrait, QueryFilter, Set};
use serde_derive::Deserialize;
use utoipa::ToSchema;
use uuid::Uuid;
use crate::{ctx::Context, error::Error, model::{page::{Page, PageParams}, prelude::Projects, projects::{self, ActiveModel, Model}}, state::AppState};

/// Search for a set of Projects.
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
    let mut page: Page<Model> = Page::new();
    page.pages      = paginator.num_pages().await?;
    page.index      = params.page();
    page.size       = items.len() as u64;
    page.records    = paginator.num_items().await?;
    page.items      = items;

    Ok(Json(page))
}

/// Select a specific Project.
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
#[derive(Debug, Deserialize, ToSchema)]
pub struct ProjectCreateParam {
    /// New Project's name.
    pub name: String,
    /// New Project's description.
    pub description: Option<String>,
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
    security(("fpa-security" = []))
)]
pub async fn create(context: Option<Context>, state: State<Arc<AppState>>, Json(params): Json<ProjectCreateParam>) -> Result<impl IntoResponse, Error> {
    println!("==> {:<12} - /create (Name: {:?})", "PROJECTS", params);
    let ctx = context.unwrap();
    let db = state.connection(ctx.tenant()).await?;

    let project = projects::ActiveModel {
        project: Set(Uuid::now_v7()),
        tenant: Set(ctx.tenant().clone()),
        user: Set(ctx.id().clone()),
        time: Set(Utc::now().into()),
        name: Set(params.name.to_owned()),
        description: Set(params.description.to_owned()),
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


/// Project update params.
#[derive(Debug, Deserialize, ToSchema)]
pub struct ProjectUpdateParam {
    /// Project Unique ID
    pub id: Uuid,
    /// New Project's name.
    pub name: String,
    /// New Project's description.
    pub description: Option<String>,
}

/// Update a existing Project.
#[utoipa::path(
    tag = "Projects",
    put,
    path = "/api/projects",
    responses(
        (status = OK, description = "Sucess.", body = Project),
        (status = UNAUTHORIZED, description = "User not authorized.", body = Error),
        (status = NOT_FOUND, description = "Project not founded.", body = Error),
        (status = SERVICE_UNAVAILABLE, description = "FPA Management service unavailable.", body = Error)
    ),
    security(("fpa-security" = []))
)]
pub async fn update(context: Option<Context>, state: State<Arc<AppState>>, Json(params): Json<ProjectUpdateParam>) -> Result<impl IntoResponse, Error> {
    println!("==> {:<12} - /update ({:?})", "PROJECTS", params);
    let ctx = context.unwrap();
    let db = state.connection(ctx.tenant()).await?;    

    let data: Option<Model> = Projects::find_by_id(params.id).one(&db).await?;
    let data = match data {
        Some(v) => v,
        None => return Err(Error::NotFound),
    };
    let mut data: ActiveModel = data.into();
    data.name           = Set(params.name);
    data.description    = Set(params.description);

    let data: Model = data.update(&db).await?;
    match db.commit().await {
        Ok(it) => it,
        Err(_) => return Err(Error::DatabaseTransaction),
    };

    Ok((StatusCode::OK, Json(data)))
}

/// Remove a existing Project.
#[utoipa::path(
    tag = "Projects",
    delete,
    path = "/api/projects/{id}",
    responses(
        (status = NO_CONTENT, description = "Sucess."),
        (status = UNAUTHORIZED, description = "User not authorized.", body = Error),
        (status = NOT_FOUND, description = "Project not founded.", body = Error),
        (status = SERVICE_UNAVAILABLE, description = "FPA Management service unavailable.", body = Error)
    ),
    params(
        ("id" = Uuid, Path, description = "Project Unique ID.")
    ),
    security(("fpa-security" = []))
)]
pub async fn remove(Path(id): Path<Uuid>, context: Option<Context>, state: State<Arc<AppState>>) -> Result<impl IntoResponse, Error> {
    println!("==> {:<12} - /remove (id: {:?})", "PROJECTS", id);
    let ctx = context.unwrap();
    let db = state.connection(ctx.tenant()).await?;    

    let data: Option<Model> = Projects::find_by_id(id).one(&db).await?;
    let data = match data {
        Some(v) => v,
        None => return Err(Error::NotFound),
    };
    match data.delete(&db).await {
        Ok(v) => {
            if v.rows_affected != 1 {
                return Err(Error::MultipleRowsAffected)
            }
        }
        Err(_) => return Err(Error::NotFound),
    };
    match db.commit().await {
        Ok(it) => it,
        Err(_) => return Err(Error::DatabaseTransaction),
    };

    Ok(StatusCode::NO_CONTENT)
}
