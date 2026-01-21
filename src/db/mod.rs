pub mod repo;

use sqlx::{migrate, pool::PoolOptions, PgPool};
use std::time::Duration;
use crate::config::Config;

pub async fn prepare_db(config: &Config) -> PgPool {
    let pool = PoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(5))
        .connect(&*config.database_url())
        .await
        .expect("Failed to create database connection pool.");
    migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run database migrations.");
    pool
}
