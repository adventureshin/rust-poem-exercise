use sea_orm::DatabaseConnection;

#[derive(Clone)]
pub struct AppContext {
    pub db: DatabaseConnection,
}

impl AppContext {
    pub fn new(db: DatabaseConnection) -> Self {
        AppContext { db }
    }
}
