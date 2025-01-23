use std::sync::Arc;

use axum::{
    extract::{Path, State}, response::IntoResponse, Json
};
use log::{debug, trace};
use reqwest::StatusCode;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, EntityTrait, ModelTrait, QueryFilter, Set,
};
use serde::Deserialize;
use serde_json::json;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{
    ctx::Context,
    error::{Error, ErrorResponse},
    model::{
        factors::{self, ActiveModel, Model},
        frontiers::{self, Entity as Frontiers},
        page::Page,
        prelude::*,
        sea_orm_active_enums::{FactorType, InfluenceType},
    },
    state::AppState,
};

/// Search for a set of Factor´s Adjustment for a Frontier.
#[utoipa::path(
    tag = "Factors",
    get,
    path = "/api/projects/{project}/frontiers/{frontier}factors",
    responses(
        (status = OK, description = "Success", body = factors::Model),
        (status = UNAUTHORIZED, description = "User not authorized.", body = ErrorResponse),
        (status = NOT_FOUND, description = "Project not founded.", body = ErrorResponse),
        (status = SERVICE_UNAVAILABLE, description = "FPA Management service unavailable.", body = ErrorResponse)
    ),
    params(
        ("project" = Uuid, description = "Project Unique ID."),
        ("frontier" = Uuid, Path, description = "Frontier Unique ID."),
    ),
    security(("fpa-security" = []))
)]
pub async fn list(
    Path((project, frontier)): Path<(Uuid, Uuid)>,
    context: Option<Context>,
    state: State<Arc<AppState>>,
) -> Result<impl IntoResponse, Error> {
    debug!("List all factors for a Frontier (project: {} - frontier: {})", project, frontier);

    let ctx = context.unwrap();
    let db = state.connection(ctx.tenant()).await?;

    let mut conditions = Condition::all();
    conditions = conditions.add(frontiers::Column::Frontier.eq(frontier));
    conditions = conditions.add(frontiers::Column::Project.eq(project));

    let frontier = match Frontiers::find().filter(conditions).one(&db).await.unwrap() {
        Some(v) => v,
        None => return Err(Error::NotFound),
    };

    let items = frontier.find_related(Factors).all(&db).await?;
    let mut page: Page<Model> = Page::new();
    page.pages = 1;
    page.index = 1;
    page.size = items.len() as u64;
    page.records = items.len() as u64;
    page.items = items;

    trace!("::: {:?}", json!(page));
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
    path = "/api/projects/{project}/frontiers/{frontier}/factors",
    responses(
        (status = OK, description = "Success", body = factors::Model),
        (status = UNAUTHORIZED, description = "User not authorized.", body = ErrorResponse),
        (status = NOT_FOUND, description = "Project not founded.", body = ErrorResponse),
        (status = SERVICE_UNAVAILABLE, description = "FPA Management service unavailable.", body = ErrorResponse)
    ),
    params(
        ("project" = Uuid, description = "Project Unique ID."),
        ("frontier" = Uuid, Path, description = "Frontier Unique ID."),
    ),
    security(("fpa-security" = []))
)]
pub async fn update(
    Path((project, frontier)): Path<(Uuid, Uuid)>,
    context: Option<Context>,
    state: State<Arc<AppState>>,
    Json(params): Json<FactorParam>,
) -> Result<impl IntoResponse, Error> {
    debug!(
        "Update a adjustement Factor (project: {} - frontier: {} - params: {:?})",
        project, frontier, params
    );

    let ctx = context.unwrap();
    let db = state.connection(ctx.tenant()).await?;

    let mut conditions = Condition::all();
    conditions = conditions.add(frontiers::Column::Project.eq(project));
    conditions = conditions.add(factors::Column::Frontier.eq(frontier));
    conditions = conditions.add(factors::Column::Factor.eq(params.factor));

    let data = match Factors::find()
        .inner_join(Frontiers)
        .filter(conditions)
        .one(&db)
        .await
        .unwrap()
    {
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

    trace!("::: {:?}", json!(data));
    Ok((StatusCode::OK, Json(data)))
}
