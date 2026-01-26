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
mod graphql;

use poem::{
    middleware::{AddData, AddDataEndpoint, Cors, CorsEndpoint},
    EndpointExt, Route,
    get,
    IntoResponse,
    web::Html,
    handler,
};
use poem_openapi::OpenApiService;
use async_graphql_poem::{GraphQL};
use async_graphql::http::{GraphiQLSource};
use sea_orm::DatabaseConnection;
use crate::config::Config;

pub type App =
    AddDataEndpoint<CorsEndpoint<middleware::ErrorMiddlewareImpl<Route>>, context::AppContext>;


#[handler]
async fn graphiql() -> impl IntoResponse {
    Html(r#"
  <!DOCTYPE html>
  <html>
  <head>
      <title>GraphiQL</title>
      <link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/graphiql@3.9.0/graphiql.min.css" />
      <script src="https://cdn.jsdelivr.net/npm/react@17.0.2/umd/react.production.min.js"></script>
      <script src="https://cdn.jsdelivr.net/npm/react-dom@17.0.2/umd/react-dom.production.min.js"></script>
      <script src="https://cdn.jsdelivr.net/npm/graphiql@3.9.0/graphiql.min.js"></script>
  </head>
  <body style="margin: 0;">
      <div id="graphiql" style="height: 100vh;">Loading...</div>
      <script>
          ReactDOM.render(
              React.createElement(GraphiQL, {
                  fetcher: GraphiQL.createFetcher({ url: '/graphql' })
              }),
              document.getElementById('graphiql')
          );
      </script>
  </body>
  </html>
  "#)
}


// -> impl Endpoint
pub async fn create_app(db: DatabaseConnection, config: Config) -> App {
    let schema = graphql::build_schema(db.clone());
    let ctx = context::AppContext::new(db, config.clone());
    let api = OpenApiService::new(
        (controller::AuthController, controller::UserController, controller::ChefController),
        "My App API",
        "1.0.0",
    )
    .server(format!("http://127.0.0.1:{}/api", config.server_port));
    let ui = api.swagger_ui();
    let app = Route::new()
        .nest("/api", api)
        .nest("/docs", ui)
        .at("/graphql", get(graphiql).post(GraphQL::new(schema)))
        .with(middleware::ErrorMiddleware)
        .with(Cors::new())
        .with(AddData::new(ctx));
    app
}
