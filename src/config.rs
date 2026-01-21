use std::env;

pub struct Config {
    pub port: u16,
    pub db_server: String,
    pub db_user: String,
    pub db_password: String,
    pub db_name: String,
}

impl Config {
    pub fn from_env() -> Self {
        dotenvy::dotenv().ok();

        let port = env::var("DATABASE_PORT")
            .unwrap_or_else(|_| "3000".to_string())
            .parse()
            .expect("PORT must be a number");

        Self {
            port,
            db_server: env::var("DATABASE_SERVER").expect("DATABASE_SERVER must be set"),
            db_user: env::var("DATABASE_USER").expect("DATABASE_USER must be set"),
            db_password: env::var("DATABASE_PASSWORD").expect("DATABASE_PASSWORD must be set"),
            db_name: env::var("DATABASE_DB").expect("DATABASE_DB must be set"),
        }
    }

    pub fn database_url(&self) -> String {
        format!(
            "postgres://{}:{}@{}/{}",
            self.db_user, self.db_password, self.db_server, self.db_name
        )
    }
}