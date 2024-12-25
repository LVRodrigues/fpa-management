use std::sync::Arc;

use crate::{
    configuration::Configuration,
    ctx::Context,
    error::{Error, ErrorResponse},
    model::{
        self, empiricals, factors,
        page::{Page, PageParams},
        prelude::Projects,
        projects::{self, ActiveModel, Model},
        sea_orm_active_enums::{EmpiricalType, FactorType, InfluenceType},
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
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, DatabaseTransaction, DbErr, EntityTrait, Iterable,
    ModelTrait, PaginatorTrait, QueryFilter, Set,
};
use serde_derive::Deserialize;
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
    page.pages = paginator.num_pages().await?;
    page.index = params.page();
    page.size = items.len() as u64;
    page.records = paginator.num_items().await?;
    page.items = items;

    Ok(Json(page))
}

/// Select a specific Project.
#[utoipa::path(
    tag = "Projects",
    get,
    path = "/api/projects/{id}",
    responses(
        (status = OK, description = "Success.", body = projects::Model),
        (status = UNAUTHORIZED, description = "User not authorized.", body = ErrorResponse),
        (status = NOT_FOUND, description = "Project not founded.", body = ErrorResponse),
        (status = SERVICE_UNAVAILABLE, description = "FPA Management service unavailable.", body = ErrorResponse)
    ),
    params(
        ("id" = Uuid, Path, description = "Project Unique ID.")
    ),
    security(("fpa-security" = []))
)]
pub async fn by_id(
    Path(id): Path<Uuid>,
    context: Option<Context>,
    state: State<Arc<AppState>>,
) -> Result<impl IntoResponse, Error> {
    println!("==> {:<12} - /byId (id: {:?})", "PROJECTS", id);
    let ctx = context.unwrap();
    let db = state.connection(ctx.tenant()).await?;

    let project: Option<projects::Model> = Projects::find_by_id(id).one(&db).await?;
    match project {
        Some(v) => Ok((StatusCode::OK, Json(v))),
        None => return Err(Error::NotFound),
    }
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
    println!("==> {:<12} - /create (Name: {:?})", "PROJECTS", params);
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

    match add_factors(&db, project.project.clone(), ctx.tenant().clone()).await {
        Ok(_) => (),
        Err(_) => return Err(Error::ProjectFactorCreate),
    };

    match add_empiricals(&db, project.project.clone(), ctx.tenant().clone(), config).await {
        Ok(_) => (),
        Err(_) => return Err(Error::ProjectEmpiricalCreate),
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
    let mut header = HeaderMap::new();
    header.insert("Location", location);

    Ok((StatusCode::CREATED, header, Json(project)))
}

async fn add_factors(db: &DatabaseTransaction, project: Uuid, tenant: Uuid) -> Result<(), DbErr> {
    for factor_type in FactorType::iter() {
        let factor = factors::ActiveModel {
            project: Set(project),
            tenant: Set(tenant),
            factor: Set(factor_type),
            influence: Set(InfluenceType::Absent),
        };
        factor.insert(db).await?;
    }
    Ok(())
}

async fn add_empiricals(
    db: &DatabaseTransaction,
    project: Uuid,
    tenant: Uuid,
    config: &Configuration,
) -> Result<(), DbErr> {
    let coordination = empiricals::ActiveModel {
        project: Set(project),
        tenant: Set(tenant),
        empirical: Set(EmpiricalType::Coordination),
        value: Set(config.empiricals.coordination),
    };
    coordination.insert(db).await?;

    let deployment = empiricals::ActiveModel {
        project: Set(project),
        tenant: Set(tenant),
        empirical: Set(EmpiricalType::Deployment),
        value: Set(config.empiricals.deployment),
    };
    deployment.insert(db).await?;

    let planning = empiricals::ActiveModel {
        project: Set(project),
        tenant: Set(tenant),
        empirical: Set(EmpiricalType::Planning),
        value: Set(config.empiricals.planning),
    };
    planning.insert(db).await?;

    let productivity = empiricals::ActiveModel {
        project: Set(project),
        tenant: Set(tenant),
        empirical: Set(EmpiricalType::Productivity),
        value: Set(config.empiricals.productivity),
    };
    productivity.insert(db).await?;

    let testing = empiricals::ActiveModel {
        project: Set(project),
        tenant: Set(tenant),
        empirical: Set(EmpiricalType::Testing),
        value: Set(config.empiricals.testing),
    };
    testing.insert(db).await?;

    Ok(())
}

/// Update a existing Project.
#[utoipa::path(
    tag = "Projects",
    put,
    path = "/api/projects/{id}",
    responses(
        (status = OK, description = "Success.", body = projects::Model),
        (status = UNAUTHORIZED, description = "User not authorized.", body = ErrorResponse),
        (status = NOT_FOUND, description = "Project not founded.", body = ErrorResponse),
        (status = CONFLICT, description = "The project name must be unique.", body = ErrorResponse),
        (status = SERVICE_UNAVAILABLE, description = "FPA Management service unavailable.", body = ErrorResponse)
    ),
    params(
        ("id" = Uuid, Path, description = "Project Unique ID.")
    ),
    security(("fpa-security" = []))
)]
pub async fn update(
    Path(id): Path<Uuid>,
    context: Option<Context>,
    state: State<Arc<AppState>>,
    Json(params): Json<ProjectParam>,
) -> Result<impl IntoResponse, Error> {
    println!("==> {:<12} - /update ({:?})", "PROJECTS", params);
    let ctx = context.unwrap();
    let db = state.connection(ctx.tenant()).await?;

    let data: Option<Model> = Projects::find_by_id(id).one(&db).await?;
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

    Ok((StatusCode::OK, Json(data)))
}

/// Remove a existing Project.
#[utoipa::path(
    tag = "Projects",
    delete,
    path = "/api/projects/{id}",
    responses(
        (status = NO_CONTENT, description = "Success."),
        (status = UNAUTHORIZED, description = "User not authorized.", body = ErrorResponse),
        (status = NOT_FOUND, description = "Project not founded.", body = ErrorResponse),
        (status = PRECONDITION_FAILED, description = "Project has related records.", body = ErrorResponse),
        (status = SERVICE_UNAVAILABLE, description = "FPA Management service unavailable.", body = ErrorResponse),
    ),
    params(
        ("id" = Uuid, Path, description = "Project Unique ID.")
    ),
    security(("fpa-security" = []))
)]
pub async fn remove(
    Path(id): Path<Uuid>,
    context: Option<Context>,
    state: State<Arc<AppState>>,
) -> Result<impl IntoResponse, Error> {
    println!("==> {:<12} - /remove (id: {:?})", "PROJECTS", id);
    let ctx = context.unwrap();
    let db = state.connection(ctx.tenant()).await?;

    let data: Option<Model> = Projects::find_by_id(id).one(&db).await?;
    let data = match data {
        Some(v) => v,
        None => return Err(Error::NotFound),
    };

    let empiricals = data
        .find_related(model::prelude::Empiricals)
        .all(&db)
        .await?;
    for empirical in empiricals {
        let item: model::empiricals::ActiveModel = empirical.into();
        item.delete(&db).await?;
    }

    let factors = data.find_related(model::prelude::Factors).all(&db).await?;
    for factor in factors {
        let item: model::factors::ActiveModel = factor.into();
        item.delete(&db).await?;
    }

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

    Ok(StatusCode::NO_CONTENT)
}
