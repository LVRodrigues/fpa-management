use std::sync::Arc;

use axum::{extract::{Query, State}, response::IntoResponse, Json};
use sea_orm::{EntityTrait, PaginatorTrait};
use crate::{ctx::Context, error::Error, model::{page::{Page, PageParams}, prelude::Projects, projects}, state::AppState};

#[utoipa::path(
    tag = "Projects",
    get,
    path = "/api/projects",
    responses(
        (status = OK, description = "Sucess.", body = Page),
        (status = SERVICE_UNAVAILABLE, description = "FPA Management service unavailable.")
    ),
    params(PageParams),
    security(("fpa-security" = []))
)]
pub async fn list(params: Query<PageParams>, context: Option<Context>, State(state): State<Arc<AppState>>) -> Result<impl IntoResponse, Error> {
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