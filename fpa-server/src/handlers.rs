use std::{sync::Arc, time::Duration};

use axum::{
    extract::State, http::StatusCode, middleware, response::IntoResponse, routing::get, Router,
};
use sea_orm::{DatabaseConnection, ConnectOptions, Database};

use crate::{
    auth,
    configuration::Configuration,
    ctx::Context,
    error::Error,
    state::AppState, mapper::response_mapper,
};

async fn prepare_connection(config: &Configuration) -> Result<DatabaseConnection, Error> {
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
    let connection = prepare_connection(&config).await?;
    let state = Arc::new(AppState::new(config, connection));

    Ok(Router::new().nest(
        "/api",
        Router::new()
            .to_owned()
            .route("/health", get(health))
            .layer(middleware::map_response(response_mapper))
            .route_layer(middleware::from_fn_with_state(state.clone(), auth::user_register))
            .route_layer(middleware::from_fn_with_state(state.clone(), auth::require))
            .with_state(state)
    ))
}

#[utoipa::path(
    tag = "Status",
    get,
    path = "/api/health",
    responses(
        (status = 204, description = "FPA Management service available."),
        (status = 503, description = "FPA Management service unavailable.")
    ),
)]
pub async fn health(context: Option<Context>, State(state): State<Arc<AppState>>) -> impl IntoResponse {
    println!("==> {:<12} - /health", "HANDLER");
    let _ = match state.connection(context.unwrap().tenant()).await {
        Ok(_) => return StatusCode::NO_CONTENT,
        Err(_) => return StatusCode::SERVICE_UNAVAILABLE
    };
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