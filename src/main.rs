use app::config::Config;
use poem::{listener::TcpListener, Server};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let config = Config::from_env();
    let addr = format!("0.0.0.0:{}", config.server_port);
    let pool = app::db::prepare_db(&config).await;
    let app = app::create_app(pool, config).await;
    let listener = TcpListener::bind(&addr);
    Server::new(listener).run(app).await
}
