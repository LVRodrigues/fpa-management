use axum::Router;
use std::{error::Error, net::SocketAddr};
use utoipa::OpenApi;
use utoipa_rapidoc::RapiDoc;
use utoipa_swagger_ui::SwaggerUi;

mod error;
mod auth;
mod handlers;
mod configuration;
mod jwks;

pub async fn start() -> Result<(), Box<dyn Error>> {
    let config = configuration::prepare();
    jwks::prepare(config).await?;

    let address = SocketAddr::from(([0, 0, 0, 0], 5000));

    let router = Router::new()
        .merge(SwaggerUi::new("/doc/swagger").url("/doc/openapi.json", handlers::ApiDoc::openapi()))
        .merge(RapiDoc::new("/doc/openapi.json").path("/"))
        .merge(handlers::router());

    println!("APF Server listening on {}", address);

    axum::Server::bind(&address)
        .serve(router.into_make_service())
        .await
        .unwrap();

    Ok(())
}
