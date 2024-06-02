use deadpool_redis::Pool;
use std::env;

#[derive(Debug)]
pub struct Config {
    pub server_host: String,
    pub server_port: u16,
    pub redis_url: String,
}

impl Config {
    pub fn from_env() -> Result<Self, env::VarError> {
        dotenv::dotenv().ok();
        Ok(Self {
            server_host: env::var("SERVER_HOST")?,
            server_port: env::var("SERVER_PORT")?.parse().unwrap(),
            redis_url: env::var("REDIS_URL")?,
        })
    }
}

pub fn create_redis_pool(config: &Config) -> Pool {
    let pool = deadpool_redis::Config::from_url(&config.redis_url)
        .create_pool(None)
        .expect("Cannot create Redis pool");
    pool
}
