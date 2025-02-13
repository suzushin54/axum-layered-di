use dotenvy::dotenv;
use std::env;

pub struct Config {
    pub database_url: String,
}

impl Config {
    pub fn from_env() -> Self {
        dotenv().ok();
        
        let database_url = format!(
            "mysql://{}:{}@{}:{}/{}",
            env::var("MYSQL_USER").expect("MYSQL_USER not set"),
            env::var("MYSQL_PASSWORD").expect("MYSQL_PASSWORD not set"),
            env::var("MYSQL_HOST").expect("MYSQL_HOST not set"),
            env::var("MYSQL_PORT").expect("MYSQL_PORT not set"),
            env::var("MYSQL_DATABASE").expect("MYSQL_DATABASE not set"),
        );

        Self {
            database_url,
        }
    }
} 