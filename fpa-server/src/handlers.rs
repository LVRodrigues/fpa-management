use std::{sync::Arc, time::Duration};

use axum::{
    extract::State, http::StatusCode, middleware, response::IntoResponse, routing::get, Router,
};
use sea_orm::{DatabaseConnection, ConnectOptions, Database};
use utoipa::OpenApi;

use crate::{
    auth,
    configuration::Configuration,
    ctx::Context,
    error::Error,
    state::AppState, respmapper::response_mapper,
};

#[derive(OpenApi)]
#[openapi(paths(health))]
pub struct ApiDoc;

async fn prepare_connection(config: Configuration) -> Result<DatabaseConnection, Error> {
    let dburl = format!("{}://{}:{}@{}:{}/{}",
        &config.database.engine,
        &config.database.username,
        &config.database.password,
        &config.database.server,
        &config.database.port,
        &config.database.name,
    );
    let mut options = ConnectOptions::new(dburl);
    options.max_connections(config.database.connections_max)
        .min_connections(config.database.connections_min)
        .connect_timeout(Duration::from_secs(config.database.timeout_connect))
        .acquire_timeout(Duration::from_secs(config.database.timeout_acquire))
        .idle_timeout(Duration::from_secs(config.database.timeout_idle))
        .max_lifetime(Duration::from_secs(config.database.lifetime));
    let conn = Database::connect(options.clone()).await;
    let conn = match conn {
        Ok(v) => v,
        Err(_) => return Err(Error::DatabaseConnection)
    };

    Ok(conn)
}

pub async fn router(config: Configuration) -> Result<Router, Error> {
    let connection = prepare_connection(config).await?;
    let state = Arc::new(AppState::new(connection));

    Ok(Router::new().nest(
        "/api",
        Router::new()
            .to_owned()
            .route("/health", get(health))
            .layer(middleware::map_response(response_mapper))
            .route_layer(middleware::from_fn(auth::require))
            .with_state(state)
    ))
}

#[utoipa::path(
    get,
    path = "/api/heath",
    responses(
        (status = 204, description = "FPA Management is online.")
    )
)]
pub async fn health(context: Context, State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let _ = state.connection(context.tenant())
        .await
        .ok_or(Error::DatabaseConnection)
        .unwrap();

    StatusCode::NO_CONTENT
}

// pub async fn tests(context: Context, State(state): State<Arc<AppState>>) -> impl IntoResponse {
//     println!("==> {:<12} - /tests", "HANDLER");
//     let db = state.connection(context.tenant())
//         .await
//         .ok_or(Error::DatabaseConnection)
//         .unwrap();

//     let items = match Tests::find()
//         .all(&db)
//         .await {
//         Ok(v) => Json(v),
//         Err(_) => return Err(Error::NotFound),
//     };

//     Ok((StatusCode::OK, items))
// }