use ::log::info;
use axum::{routing::get_service, Router};
use std::{error::Error, net::SocketAddr};
use tokio::{net::TcpListener, signal};
use tower_http::services::ServeDir;
use utoipa::OpenApi;
use utoipa_redoc::{Redoc, Servable};
use utoipa_swagger_ui::SwaggerUi;

mod auth;
mod configuration;
mod ctx;
mod docs;
mod error;
mod handlers;
mod jwks;
mod log;
mod mapper;
mod model;
mod state;

pub async fn start() -> Result<(), Box<dyn Error>> {
    let config = configuration::prepare();

    let router = Router::new()
        .merge(SwaggerUi::new("/doc/swagger").url("/doc/openapi.json", docs::ApiDoc::openapi()))
        //.merge(RapiDoc::new("/doc/openapi.json").path("/doc/rapidoc"))
        .merge(Redoc::with_url("/", docs::ApiDoc::openapi()))
        .nest_service("/assets", get_service(ServeDir::new("./assets")))
        .merge(handlers::router(config.clone()).await.unwrap());

    let address = SocketAddr::from(([0, 0, 0, 0], config.port));
    info!("APF Server listening on {}", address);

    let listener = TcpListener::bind(address).await.unwrap();
    axum::serve(listener, router.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();

    Ok(())
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("Failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("Failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    info!("Finishing service...");
}
