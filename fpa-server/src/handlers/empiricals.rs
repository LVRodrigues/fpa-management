
use std::sync::Arc;

use sea_orm::{ActiveModelTrait, EntityTrait, ModelTrait, Set};
use serde::Deserialize;
use utoipa::ToSchema;
use uuid::Uuid;
use axum::{extract::{Path, State}, http::StatusCode, response::IntoResponse, Json};

use crate::{ctx::Context, error::{Error, ErrorResponse}, model::{empiricals::{self, ActiveModel, Model}, page::Page, prelude::{Empiricals, Projects}, sea_orm_active_enums::EmpiricalType}, state::AppState};

/// Search for a set of Empirical's Factor for a Project.
#[utoipa::path(
    tag = "Empiricals",
    get,
    path = "/api/projects/{id}/empiricals",
    responses(
        (status = OK, description = "Success", body = empiricals::Model),
        (status = UNAUTHORIZED, description = "User not authorized.", body = ErrorResponse),
        (status = NOT_FOUND, description = "Project not founded.", body = ErrorResponse),
        (status = SERVICE_UNAVAILABLE, description = "FPA Management service unavailable.", body = ErrorResponse)
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

/// Empirical's properties.
#[derive(Debug, Deserialize, ToSchema)]
pub struct EmpiricalParam {
    /// Empirical`s Factor
    pub empirical: EmpiricalType,
    /// Percent of influence for the Empirical`s Factor.
    pub value: i32,
}

/// Update a Empirical Factor.
#[utoipa::path(
    tag = "Empiricals",
    put,
    path = "/api/projects/{id}/empiricals",
    responses(
        (status = OK, description = "Success", body = empiricals::Model),
        (status = UNAUTHORIZED, description = "User not authorized.", body = ErrorResponse),
        (status = NOT_FOUND, description = "Project not founded.", body = ErrorResponse),
        (status = SERVICE_UNAVAILABLE, description = "FPA Management service unavailable.", body = ErrorResponse)
    ),
    params(
        ("id" = Uuid, description = "Project Unique ID.")
    ),
    security(("fpa-security" = []))
)]
pub async fn update(Path(id): Path<Uuid>, context: Option<Context>, state: State<Arc<AppState>>, Json(params): Json<EmpiricalParam>) -> Result<impl IntoResponse, Error> {
    println!("==> {:<12} - /{id}/update (Params: {:?})", "EMPIRICALS", params);

    if params.empirical == EmpiricalType::Productivity {
        if params.value < 1 || params.value > 50 {
            return Err(Error::ProductivityInvalid);
        }
    } else {
        if params.value < 0 || params.value > 100 {
            return Err(Error::EmpiricalInvalid);
        }
    }

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