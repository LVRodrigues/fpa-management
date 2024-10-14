
use std::sync::Arc;

use sea_orm::{ActiveModelTrait, EntityTrait, ModelTrait, Set};
use serde::Deserialize;
use utoipa::ToSchema;
use uuid::Uuid;
use axum::{extract::{Path, Query, State}, http::{HeaderMap, StatusCode, Uri}, response::IntoResponse, Json};
use chrono::Utc;

use crate::{ctx::Context, error::Error, model::{empiricals::{self, ActiveModel, Model}, page::Page, prelude::{Empiricals, Projects}, sea_orm_active_enums::EmpiricalType}, state::AppState};

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
    println!("==> {:<12} - /{id}/list", "EMPIRICALS");
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

/// Empirical update params.
#[derive(Debug, Deserialize, ToSchema)]
pub struct EmpiricalUpdateParam {
    /// Empirical`s Factor
    pub empirical: EmpiricalType,
    /// Percent of influence for the Empirical`s Factor.
    pub value: i32,
}

#[utoipa::path(
    tag = "Empiricals",
    put,
    path = "/api/projects/{id}/empiricals"    ,
    responses(
        (status = OK, description = "Success", body = Empirical),
        (status = UNAUTHORIZED, description = "User not authorized.", body = Error),
        (status = NOT_FOUND, description = "Project not founded.", body = Error),
        (status = SERVICE_UNAVAILABLE, description = "FPA Management service unavailable.", body = Error)
    ),
    params(
        ("id" = Uuid, description = "Project Unique ID.")
    ),
    security(("fpa-security" = []))
)]
pub async fn update(Path(id): Path<Uuid>, context: Option<Context>, state: State<Arc<AppState>>, Json(params): Json<EmpiricalUpdateParam>) -> Result<impl IntoResponse, Error> {
    println!("==> {:<12} - /{id}/update (Params: {:?})", "EMPIRICALS", params);
    let ctx = context.unwrap();
    let db = state.connection(ctx.tenant()).await?;    

    let data = match Empiricals::find_by_id((id, params.empirical)).one(&db).await.unwrap() {
        Some(v) => v,
        None => return Err(Error::NotFound),
    };

    let mut data: ActiveModel = data.into();
    data.value = Set(params.value);
    
    let data: Model = data.update(&db).await?;
    match db.commit().await {
        Ok(it) => it,
        Err(_) => return Err(Error::DatabaseTransaction),
    };

    Ok((StatusCode::OK, Json(data)))
}