use std::{error::Error, path::Path};

use log::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    log4rs::init_file(Path::new("log4rs.yaml"), Default::default()).unwrap();
    info!("Starting fpa-server...");

    fpa_server::start().await.unwrap();

    Ok(())
}
