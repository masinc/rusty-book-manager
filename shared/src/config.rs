#[derive(Debug)]
pub struct DatabaseConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub database: String,
}

#[derive(Debug)]
pub struct AppConfig {
    pub database: DatabaseConfig,
}

impl AppConfig {
    pub fn new() -> anyhow::Result<Self> {
        use std::env::var;
        let database = DatabaseConfig {
            host: var("DATABASE_HOST")?,
            port: var("DATABASE_PORT")?.parse()?,
            username: var("DATABASE_USERNAME")?,
            password: var("DATABASE_PASSWORD")?,
            database: var("DATABASE_NAME")?,
        };

        Ok(Self { database })
    }
}
