use sea_orm::{DatabaseConnection, ConnectionTrait, DatabaseTransaction, TransactionTrait};
use uuid::Uuid;


#[derive(Clone, Debug)]
pub struct AppState {
    connection: DatabaseConnection,
}

impl AppState {
    pub fn new(connection: DatabaseConnection) -> Self {
        Self {connection}
    }

    pub async fn connection(&self, tenant: &Uuid) -> Option<DatabaseTransaction> {
        let db = &self.connection;
        if db.ping().await.is_err() {
            return None
        }

        let trx = db.begin().await.unwrap();

        let sql = format!("SET app.current_tenant = '{}';", tenant);
        if trx.execute_unprepared(sql.as_str()).await.is_err() {
            return None
        }

        Some(trx)
    }
}
