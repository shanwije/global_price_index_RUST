use std::env;

#[derive(Debug)]
pub struct Config {
    pub server_host: String,
    pub server_port: u16,
}

impl Config {
    pub fn from_env() -> Result<Self, env::VarError> {
        dotenv::dotenv().ok();
        Ok(Self {
            server_host: env::var("SERVER_HOST")?,
            server_port: env::var("SERVER_PORT")?.parse().unwrap(),
        })
    }
}
