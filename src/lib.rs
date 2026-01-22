pub mod common;
pub mod config;
pub mod context;
pub mod controller;
pub mod db;
pub mod middleware;
pub mod response;
pub mod scheme;
pub mod service;
pub mod entity;

use poem::{
    middleware::{AddData, AddDataEndpoint, Cors, CorsEndpoint},
    EndpointExt, Route,
};
use poem_openapi::OpenApiService;
use sea_orm::DatabaseConnection;

pub type App =
    AddDataEndpoint<CorsEndpoint<middleware::ErrorMiddlewareImpl<Route>>, context::AppContext>;

// -> impl Endpoint
pub async fn create_app(db: DatabaseConnection) -> App {
    let ctx = context::AppContext::new(db);
    let api = OpenApiService::new(
        (controller::AuthController, controller::UserController),
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION"),
    )
    .server("http://127.0.0.1:3000/api");
    let ui = api.swagger_ui();
    let app = Route::new()
        .nest("/api", api)
        .nest("/", ui)
        .with(middleware::ErrorMiddleware)
        .with(Cors::new())
        .with(AddData::new(ctx));
    app
}
