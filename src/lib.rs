pub mod common;
pub mod config;
pub mod context;
pub mod controller;
pub mod db;
pub mod entity;
pub mod middleware;
pub mod response;
pub mod scheme;
pub mod service;

use poem::{
    middleware::{AddData, AddDataEndpoint, Cors, CorsEndpoint},
    EndpointExt, Route,
};
use poem_openapi::OpenApiService;
use sea_orm::DatabaseConnection;
use crate::config::Config;

pub type App =
    AddDataEndpoint<CorsEndpoint<middleware::ErrorMiddlewareImpl<Route>>, context::AppContext>;

// -> impl Endpoint
pub async fn create_app(db: DatabaseConnection, config: Config) -> App {
    let ctx = context::AppContext::new(db, config.clone());
    let api = OpenApiService::new(
        (controller::AuthController, controller::UserController, controller::ChefController),
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION"),
    )
    .server(format!("http://127.0.0.1:{}/api", config.server_port));
    let ui = api.swagger_ui();
    let app = Route::new()
        .nest("/api", api)
        .nest("/docs", ui)
        .with(middleware::ErrorMiddleware)
        .with(Cors::new())
        .with(AddData::new(ctx));
    app
}
