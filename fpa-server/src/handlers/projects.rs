use std::sync::Arc;

use axum::{extract::{Query, State}, http::{HeaderMap, StatusCode, Uri}, response::IntoResponse, Json};
use chrono::Utc;
use sea_orm::{ActiveModelTrait, EntityTrait, PaginatorTrait, Set};
use serde_derive::Deserialize;
use utoipa::IntoParams;
use uuid::Uuid;
use crate::{ctx::Context, error::Error, model::{page::{Page, PageParams}, prelude::Projects, projects}, state::AppState};

#[utoipa::path(
    tag = "Projects",
    get,
    path = "/api/projects",
    responses(
        (status = OK, description = "Sucess.", body = Projects),
        (status = UNAUTHORIZED, description = "User not authorized."),
        (status = SERVICE_UNAVAILABLE, description = "FPA Management service unavailable.")
    ),
    params(PageParams),
    security(("fpa-security" = []))
)]
pub async fn list(params: Query<PageParams>, context: Option<Context>, state: State<Arc<AppState>>) -> Result<impl IntoResponse, Error> {
    println!("==> {:<12} - /list (Page: {:?} - Size: {:?})", "PROJECTS", params.page(), params.size());
    
    let db = state.connection(context.unwrap().tenant()).await?;
    let paginator = Projects::find()
        .paginate(&db, params.size());

    let items = paginator.fetch_page(params.page() - 1).await?;
    let mut page: Page<projects::Model> = Page::new();
    page.total      = paginator.num_pages().await?;
    page.index      = params.page();
    page.size       = items.len() as u64;
    page.records    = paginator.num_items().await?;
    page.items      = items;

    Ok(Json(page))
}

/// Project create params.
#[derive(Debug, Deserialize, IntoParams)]
pub struct ProjectCreateParam {
    /// New Project' name.
    pub name: String,
}

#[utoipa::path(
    tag = "Projects",
    post,
    path = "/api/projects",
    responses(
        (status = CREATED, description = "Sucess."),
        (status = UNAUTHORIZED, description = "User not authorized."),
        (status = SERVICE_UNAVAILABLE, description = "FPA Management service unavailable.")
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

    Ok((StatusCode::CREATED, header))
}