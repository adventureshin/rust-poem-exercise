pub mod repo;

use std::time::Duration;
use sea_orm::{Database, DatabaseConnection, ConnectOptions};
use crate::config::Config;

pub async fn prepare_db(config: &Config) -> DatabaseConnection {
    let mut opt = ConnectOptions::new(&config.database_url());
    opt.max_connections(5)
        .acquire_timeout(Duration::from_secs(5));

    Database::connect(opt)
        .await
        .expect("Failed to create database connection.")
}

