pub mod empiricals;
pub mod factors;
pub mod functions;
pub mod modules;
pub mod projects;

use std::{sync::Arc, time::Duration};

use axum::{
    extract::State, http::StatusCode, middleware, response::IntoResponse, routing::get, Router,
};
use sea_orm::{ConnectOptions, Database, DatabaseConnection};

use crate::{
    auth, configuration::Configuration, ctx::Context, error::Error, mapper::response_mapper,
    state::AppState,
};

async fn prepare_connection(config: &Configuration) -> Result<DatabaseConnection, Error> {
    let dburl = format!(
        "{}://{}:{}@{}:{}/{}",
        &config.database.engine,
        &config.database.username,
        &config.database.password,
        &config.database.server,
        &config.database.port,
        &config.database.name,
    );
    let mut options = ConnectOptions::new(dburl);
    options
        .max_connections(config.database.connections_max)
        .min_connections(config.database.connections_min)
        .connect_timeout(Duration::from_secs(config.database.timeout_connect))
        .acquire_timeout(Duration::from_secs(config.database.timeout_acquire))
        .idle_timeout(Duration::from_secs(config.database.timeout_idle))
        .max_lifetime(Duration::from_secs(config.database.lifetime));
    let conn = Database::connect(options.clone()).await;
    let conn = match conn {
        Ok(v) => v,
        Err(_) => return Err(Error::DatabaseConnection),
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
            .route("/projects", get(projects::list).post(projects::create))
            .route(
                "/projects/:project",
                get(projects::by_id)
                    .delete(projects::remove)
                    .put(projects::update),
            )
            .route(
                "/projects/:project/empiricals",
                get(empiricals::list).put(empiricals::update),
            )
            .route(
                "/projects/:project/factors",
                get(factors::list).put(factors::update),
            )
            .route(
                "/projects/:project/modules",
                get(modules::list).post(modules::create),
            )
            .route(
                "/projects/:project/modules/:module",
                get(modules::by_id)
                    .put(modules::update)
                    .delete(modules::remove),
            )
            .route(
                "/projects/:project/modules/:module/functions",
                get(functions::list).post(functions::create),
            )
            .route(
                "/projects/:project/modules/:module/functions/:function",
                get(functions::by_id).put(functions::update),
            )
            .route("/health", get(health))
            .layer(middleware::map_response(response_mapper))
            .route_layer(middleware::from_fn_with_state(
                state.clone(),
                auth::user_register,
            ))
            .route_layer(middleware::from_fn_with_state(state.clone(), auth::require))
            .with_state(state),
    ))
}

/// Checks system health.
#[utoipa::path(
    tag = "Status",
    get,
    path = "/api/health",
    responses(
        (status = NO_CONTENT, description = "FPA Management service available."),
        (status = UNAUTHORIZED, description = "User not authorized."),
        (status = SERVICE_UNAVAILABLE, description = "FPA Management service unavailable."),
    ),
    security(("fpa-security" = []))
)]
pub async fn health(context: Option<Context>, state: State<Arc<AppState>>) -> impl IntoResponse {
    println!("==> {:<12} - /health", "HANDLER");
    let _ = match state.connection(context.unwrap().tenant()).await {
        Ok(_) => return StatusCode::NO_CONTENT,
        Err(_) => return StatusCode::SERVICE_UNAVAILABLE,
    };
}
