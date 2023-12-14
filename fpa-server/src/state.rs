use std::time::Duration;

use sea_orm::{DatabaseConnection, ConnectionTrait, ConnectOptions, Database};

use crate::{configuration::Configuration, error::Error};

#[derive(Clone, Debug)]
pub struct AppState {
    connection: DatabaseConnection,
}

impl AppState {
    pub fn new(connection: DatabaseConnection) -> Self {
        Self {connection}
    }

    pub async fn connection(&self, tenant: &str) -> Option<&DatabaseConnection> {
        let db = &self.connection;
        if db.ping().await.is_err() {
            return None
        }

        let sql = format!("SET app.current_tenant = '{}';", tenant);
        if db.execute_unprepared(sql.as_str()).await.is_err() {
            return None
        }

        Some(db)
    }
}

async fn prepare_connection(config: Configuration) -> Result<DatabaseConnection, Error> {
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