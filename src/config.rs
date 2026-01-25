use std::env;

#[derive(Clone)]
pub struct Config {
    pub server_port: u16,
    pub db_port: u16,
    pub db_server: String,
    pub db_user: String,
    pub db_password: String,
    pub db_name: String,
    pub google_client_id: String,
    pub google_client_secret: String,
    pub google_redirect_url: String,
}

impl Config {
    pub fn from_env() -> Self {
        dotenvy::dotenv().ok();
        let server_port = env::var("SERVER_PORT")
            .unwrap_or_else(|_| "8000".to_string())
            .parse()
            .expect("SERVER PORT must be a number");

        let db_port = env::var("DATABASE_PORT")
            .unwrap_or_else(|_| "3000".to_string())
            .parse()
            .expect("PORT must be a number");

        Self {
            server_port,
            db_port,
            db_server: env::var("DATABASE_SERVER").expect("DATABASE_SERVER must be set"),
            db_user: env::var("DATABASE_USER").expect("DATABASE_USER must be set"),
            db_password: env::var("DATABASE_PASSWORD").expect("DATABASE_PASSWORD must be set"),
            db_name: env::var("DATABASE_DB").expect("DATABASE_DB must be set"),
            google_client_id: env::var("GOOGLE_CLIENT_ID").expect("GOOGLE_CLIENT_ID must be set"),
            google_client_secret: env::var("GOOGLE_CLIENT_SECRET")
                .expect("GOOGLE_CLIENT_SECRET must be set"),
            google_redirect_url: env::var("GOOGLE_REDIRECT_URL")
                .expect("GOOGLE_REDIRECT_URL must be set"),
        }
    }

    pub fn database_url(&self) -> String {
        format!(
            "postgres://{}:{}@{}/{}",
            self.db_user, self.db_password, self.db_server, self.db_name
        )
    }
}
