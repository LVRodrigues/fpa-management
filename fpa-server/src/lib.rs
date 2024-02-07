use axum::Router;
use tokio::{net::TcpListener, signal};
use std::{error::Error, net::SocketAddr};
use utoipa::{openapi::Components, OpenApi};
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

use utoipa::{openapi::security::{Flow, OAuth2, Password, Scopes, SecurityScheme}, Modify};

pub async fn start() -> Result<(), Box<dyn Error>> {
    #[derive(OpenApi)]
    #[openapi(
        paths(handlers::health),
        tags([name = "Status", description = "Check service health."]),
        info(description = "Project Management using Function Points Analysis."),
        modifiers(&SecuritySchemas),
    )]
    struct ApiDoc;    

    struct SecuritySchemas;
    impl Modify for SecuritySchemas {

        fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
            let components = match openapi.components.as_mut() {
                Some(c) => c,
                None => {
                    openapi.components = Some(Components::new());
                    openapi.components.as_mut().unwrap()
                },
            };
            let flows = [Flow::Password(
                Password::new("http://localhost:8080/realms/default/protocol/openid-connect/token", Scopes::default())
            )];
            let oauth2 = OAuth2::new(flows);
            let scheme = SecurityScheme::OAuth2(oauth2);
            components.add_security_scheme("fpa-security", scheme);
        }
    }

    let config = configuration::prepare();

    let router = Router::new()
        .merge(SwaggerUi::new("/doc/swagger").url("/doc/openapi.json", ApiDoc::openapi()))
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