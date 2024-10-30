use std::sync::Arc;

use sea_orm::{ActiveModelTrait, ColumnTrait, Condition, EntityTrait, ModelTrait, PaginatorTrait, QueryFilter, Set};
use serde::Deserialize;
use utoipa::ToSchema;
use uuid::Uuid;
use axum::{extract::{Path, Query, State}, http::{HeaderMap, Uri, StatusCode}, response::IntoResponse, Json};

use crate::{ctx::Context, error::{Error, ErrorResponse}, model::{modules::{self, ActiveModel, Model}, page::{Page, PageParams}}, state::AppState};
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
    page.pages      = paginator.num_pages().await?;
    page.index      = params.page();
    page.size       = items.len() as u64;
    page.records    = paginator.num_items().await?;
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
pub async fn by_id(Path((id, module)): Path<(Uuid, Uuid)>, context: Option<Context>, state: State<Arc<AppState>>) -> Result<impl IntoResponse, Error> {
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

/// Module's properties.
#[derive(Debug, Deserialize, ToSchema)]
pub struct ModuleParam {
    /// Module's name.
    pub name: String,
    /// Description for the Module.
    pub description: Option<String>,
}

/// Create a new Module for a selected Project.
#[utoipa::path(
    tag = "Modules",
    post,
    path = "/api/projects/{id}/modules",
    responses(
        (status = CREATED, description = "Success.", body = modules::Model, headers(("Location", description = "New module address."))),
        (status = UNAUTHORIZED, description = "User not authorized.", body = ErrorResponse),
        (status = CONFLICT, description = "The name must be unique for the selected project.", body = ErrorResponse),
        (status = SERVICE_UNAVAILABLE, description = "FPA Management service unavailable.", body = ErrorResponse)
    ),
    security(("fpa-security" = []))
)]
pub async fn create(Path(id): Path<Uuid>, context: Option<Context>, state: State<Arc<AppState>>, Json(params): Json<ModuleParam>) -> Result<impl IntoResponse, Error> {
    println!("==> {:<12} - /{id}/create {:?}", "MODULES", params);
    let ctx = context.unwrap();
    let db = state.connection(ctx.tenant()).await?;
    let config = state.configuration();

    let module = modules::ActiveModel {
        project: Set(id.clone()),
        tenant: Set(ctx.tenant().clone()),
        module: Set(Uuid::now_v7()),
        name: Set(params.name.to_owned()),
        description: Set(params.description.to_owned()),
    };
    let module = match module.insert(&db).await {
        Ok(v) => v,
        Err(e) => {
            match e.sql_err().unwrap() {
                sea_orm::SqlErr::UniqueConstraintViolation(_) => return Err(Error::ModuleNameDuplicated),
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
        .authority(format!("{}:{}", config.authority.clone(), config.port.clone()))
        .path_and_query(format!("/api/projects/{}/modules/{}", &module.project, module.module))
        .build()
        .unwrap()
        .to_string()
        .parse()
        .unwrap();
    let mut header = HeaderMap::new();
    header.insert("Location", location);

    Ok((StatusCode::CREATED, header, Json(module)))
}    

/// Update a existing Module.
#[utoipa::path(
    tag = "Modules",
    put,
    path = "/api/projects/{id}/modules/{module}",
    responses(
        (status = OK, description = "Success.", body = modules::Model),
        (status = UNAUTHORIZED, description = "User not authorized.", body = ErrorResponse),
        (status = NOT_FOUND, description = "Project or Module not founded.", body = ErrorResponse),
        (status = CONFLICT, description = "The name must be unique for the selected project.", body = ErrorResponse),
        (status = SERVICE_UNAVAILABLE, description = "FPA Management service unavailable.", body = ErrorResponse)
    ),
    params(
        ("id" = Uuid, Path, description = "Project Unique ID."),
        ("module" = Uuid, Path, description = "Module Unique ID."),
    ),
    security(("fpa-security" = []))
)]
pub async fn update(Path((id, module)): Path<(Uuid, Uuid)>, context: Option<Context>, state: State<Arc<AppState>>, Json(params): Json<ModuleParam>) -> Result<impl IntoResponse, Error> {
    println!("==> {:<12} - /{id}/update/{module} {:?}", "MODULES", params);
    let ctx = context.unwrap();
    let db = state.connection(ctx.tenant()).await?;        

    let mut conditions = Condition::all();
    conditions = conditions.add(modules::Column::Project.eq(id));
    conditions = conditions.add(modules::Column::Module.eq(module));

    let data = Modules::find().filter(conditions).one(&db).await?;
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
                sea_orm::SqlErr::UniqueConstraintViolation(_) => return Err(Error::ModuleNameDuplicated),
                _ => return Err(Error::ModuleUpdate)
            };
        }
    };

    Ok((StatusCode::OK, Json(data)))
}

/// Remove a existing Module.
#[utoipa::path(
    tag = "Modules",
    delete,
    path = "/api/projects/{id}/modules/{module}",
    responses(
        (status = NO_CONTENT, description = "Success."),
        (status = UNAUTHORIZED, description = "User not authorized.", body = ErrorResponse),
        (status = NOT_FOUND, description = "Project or Module not founded.", body = ErrorResponse),
        (status = PRECONDITION_FAILED, description = "Module has related records.", body = ErrorResponse),
        (status = SERVICE_UNAVAILABLE, description = "FPA Management service unavailable.", body = ErrorResponse),
    ),
    params(
        ("id" = Uuid, Path, description = "Project Unique ID."),
        ("module" = Uuid, Path, description = "Module Unique ID."),
    ),
    security(("fpa-security" = []))
)]
pub async fn remove(Path((id, module)): Path<(Uuid, Uuid)>, context: Option<Context>, state: State<Arc<AppState>>) -> Result<impl IntoResponse, Error> {
    println!("==> {:<12} - /{id}/remove/{module}", "MODULES");
    let ctx = context.unwrap();
    let db = state.connection(ctx.tenant()).await?;   

    let mut conditions = Condition::all();
    conditions = conditions.add(modules::Column::Project.eq(id));
    conditions = conditions.add(modules::Column::Module.eq(module));

    let data = Modules::find().filter(conditions).one(&db).await?;
    let data = match data {
        Some(v) => v,
        None => return Err(Error::NotFound),
    };

    match data.delete(&db).await {
        Ok(v) => {
            if v.rows_affected != 1 {
                return Err(Error::MultipleRowsAffected)
            }
        },
        Err(_) => return Err(Error::ModuleConstraints),
    };
    match db.commit().await {
        Ok(it) => it,
        Err(_) => return Err(Error::DatabaseTransaction),
    }

    Ok(StatusCode::NO_CONTENT)
}