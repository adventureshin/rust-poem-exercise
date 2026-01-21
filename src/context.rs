use sqlx::{PgPool};

#[derive(Clone)]
pub struct AppContext {
    pub db_pool: PgPool,
}

impl AppContext {
    pub fn new(db_pool: PgPool) -> Self {
        AppContext { db_pool }
    }
}
