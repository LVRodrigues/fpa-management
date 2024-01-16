use sea_orm::{DatabaseConnection, ConnectionTrait, DatabaseTransaction, TransactionTrait};
use uuid::Uuid;

use crate::error::Error;


#[derive(Clone, Debug)]
pub struct AppState {
    connection: DatabaseConnection,
}

impl AppState {
    pub fn new(connection: DatabaseConnection) -> Self {
        Self {connection}
    }

    pub async fn connection(&self, tenant: &Uuid) -> Result<DatabaseTransaction, Error> {
        println!("==> {:<12} - connection", "DATABASE");
        let db = &self.connection;
        if db.ping().await.is_err() {
            return Err(Error::DatabaseConnection)
        }

        let trx = db.begin().await.unwrap();

        let sql = format!("SET app.current_tenant = '{}';", tenant);
        if trx.execute_unprepared(sql.as_str()).await.is_err() {
            return Err(Error::DatabaseConnection)
        }

        Ok(trx)
    }
}
