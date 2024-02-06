use axum::Router;
use tokio::{net::TcpListener, signal};
use std::{error::Error, net::SocketAddr};
use utoipa::OpenApi;
use utoipa_rapidoc::RapiDoc;
use utoipa_swagger_ui::SwaggerUi;

mod configuration;
mod error;
mod jwks;
mod auth;
mod ctx;
mod state;
mod handlers;
mod model;
mod mapper;
mod log;

pub async fn start() -> Result<(), Box<dyn Error>> {
    let config = configuration::prepare();
    //jwks::prepare(config.clone()).await?;

    let router = Router::new()
        .merge(SwaggerUi::new("/doc/swagger").url("/doc/openapi.json", handlers::ApiDoc::openapi()))
        .merge(RapiDoc::new("/doc/openapi.json").path("/"))
        .merge(handlers::router(config.clone()).await.unwrap());
    
    let address = SocketAddr::from(([0, 0, 0, 0], config.port));
    println!("APF Server listening on {}", address);
    let listener = TcpListener::bind(address)
        .await
        .unwrap();
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

    println!("Finishing service...");
}