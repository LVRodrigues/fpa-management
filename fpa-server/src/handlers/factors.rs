use std::sync::Arc;

use axum::{extract::{Path, State}, response::IntoResponse, Json};
use reqwest::StatusCode;
use sea_orm::{ActiveModelTrait, EntityTrait, ModelTrait, Set};
use serde::Deserialize;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{ctx::Context, error::{Error, ErrorResponse}, model::{factors::{self, ActiveModel, Model}, page::Page, prelude::{Factors, Projects}, sea_orm_active_enums::{FactorType, InfluenceType}}, state::AppState};


/// Search for a set of Factor´s Adjustment for a Project.
#[utoipa::path(
    tag = "Factors",
    get,
    path = "/api/projects/{id}/factors",
    responses(
        (status = OK, description = "Success", body = factors::Model),
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
    println!("==> {:<12} - /{id}/list", "FACTORS");
    let ctx = context.unwrap();
    let db = state.connection(ctx.tenant()).await?;

    let project = match Projects::find_by_id(id).one(&db).await.unwrap() {
        Some(v) => v,
        None => return Err(Error::NotFound),
    };

    let items = project.find_related(Factors).all(&db).await?;
    let mut page: Page<Model> = Page::new();
    page.pages      = 1;
    page.index      = 1;
    page.size       = items.len() as u64;
    page.records    = items.len() as u64;
    page.items      = items;

    Ok(Json(page))
}

/// Adjustments Factors's properties.
#[derive(Debug, Deserialize, ToSchema)]
pub struct FactorParam {
    /// Adjustment Fator for the Project.
    pub factor: FactorType,
    /// Influence value for the factor on this project.
    pub influence: InfluenceType,    
}

/// Update a adjustement Factor.
#[utoipa::path(
    tag = "Factors",
    put,
    path = "/api/projects/{id}/factors",
    responses(
        (status = OK, description = "Success", body = factors::Model),
        (status = UNAUTHORIZED, description = "User not authorized.", body = ErrorResponse),
        (status = NOT_FOUND, description = "Project not founded.", body = ErrorResponse),
        (status = SERVICE_UNAVAILABLE, description = "FPA Management service unavailable.", body = ErrorResponse)
    ),
    params(
        ("id" = Uuid, description = "Project Unique ID.")
    ),
    security(("fpa-security" = []))
)]
pub async fn update(Path(id): Path<Uuid>, context: Option<Context>, state: State<Arc<AppState>>, Json(params): Json<FactorParam>) -> Result<impl IntoResponse, Error> {
    println!("==> {:<12} - /{id}/update (Params: {:?}", "FACTORS", params);
    let ctx = context.unwrap();
    let db = state.connection(ctx.tenant()).await?;    

    let data = match Factors::find_by_id((id, params.factor)).one(&db).await.unwrap() {
        Some(v) => v,
        None => return Err(Error::NotFound),
    };

    let mut data: ActiveModel = data.into();
    data.influence = Set(params.influence);

    let data: Model = data.update(&db).await?;
    match db.commit().await {
        Ok(it) => it,
        Err(_) => return Err(Error::DatabaseTransaction),
    }

    Ok((StatusCode::OK, Json(data)))
}