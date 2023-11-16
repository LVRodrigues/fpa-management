use std::{error::Error, net::SocketAddr};
use axum::{
    routing::get, 
    // routing::post, 
    // routing::put, 
    // routing::delete, 
    Router
};

mod handlers;

pub async fn start() -> Result<(), Box<dyn Error>> {
    let address = SocketAddr::from(([0, 0, 0, 0], 8000));

	let router = Router::new().route(
        "/", get(handlers::hello)
    );

    println!("APF Server listening on {}", address);

    axum::Server::bind(&address)
        .serve(router.into_make_service())
        .await
        .unwrap();

    Ok(())   
}
