use sea_orm::DatabaseConnection;
use crate::config::Config;

#[derive(Clone)]
pub struct AppContext {
    pub db: DatabaseConnection,
    pub config: Config,
}

impl AppContext {
    pub fn new(db: DatabaseConnection, config: Config) -> Self {
        AppContext { db, config }
    }
}
