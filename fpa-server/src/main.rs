pub mod state;

use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    fpa::start()
        .await
        .unwrap();

    Ok(())
}
