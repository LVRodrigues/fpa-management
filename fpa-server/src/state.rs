use log::trace;
use sea_orm::{ConnectionTrait, DatabaseConnection, DatabaseTransaction, TransactionTrait};
use uuid::Uuid;

use crate::{configuration::Configuration, error::Error};

#[derive(Clone, Debug)]
pub struct AppState {
    configuration: Configuration,
    connection: DatabaseConnection,
}

impl AppState {
    pub fn new(configuration: Configuration, connection: DatabaseConnection) -> Self {
        Self {
            configuration,
            connection,
        }
    }

    pub async fn connection(&self, tenant: &Uuid) -> Result<DatabaseTransaction, Error> {
        trace!("New database connection.");
        let db = &self.connection;
        if db.ping().await.is_err() {
            return Err(Error::DatabaseConnection);
        }

        let trx = db.begin().await.unwrap();

        let sql = format!("SET app.current_tenant = '{}';", tenant);
        if trx.execute_unprepared(sql.as_str()).await.is_err() {
            return Err(Error::DatabaseConnection);
        }

        Ok(trx)
    }

    pub fn configuration(&self) -> &Configuration {
        &self.configuration
    }
}
