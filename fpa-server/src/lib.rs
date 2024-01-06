use axum::{Router, middleware};
use tokio::net::TcpListener;
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
mod respmapper;
mod log;

pub async fn start() -> Result<(), Box<dyn Error>> {
    let config = configuration::prepare();
    jwks::prepare(config.clone()).await?;

    let router = Router::new()
        .merge(SwaggerUi::new("/doc/swagger").url("/doc/openapi.json", handlers::ApiDoc::openapi()))
        .merge(RapiDoc::new("/doc/openapi.json").path("/"))
        .merge(handlers::router(config.clone()).await.unwrap());
    
    let address = SocketAddr::from(([0, 0, 0, 0], 5000));
    println!("APF Server listening on {}", address);
    let listener = TcpListener::bind(address)
        .await
        .unwrap();
    axum::serve(listener, router.into_make_service())
        .await
        .unwrap();

    Ok(())
}
