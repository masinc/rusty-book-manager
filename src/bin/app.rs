use std::net::{Ipv4Addr, SocketAddr};

use axum::extract::State;
use axum::http::StatusCode;
use axum::{routing::get, Router};
use sqlx::postgres::PgConnectOptions;
use sqlx::PgPool;
use tokio::net::TcpListener;

#[derive(Debug)]
struct DatabaseConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub database: String,
}

impl DatabaseConfig {
    pub fn connect(&self) -> PgPool {
        let options: PgConnectOptions = self.into();
        sqlx::Pool::connect_lazy_with(options)
    }
}

impl From<&DatabaseConfig> for PgConnectOptions {
    fn from(config: &DatabaseConfig) -> Self {
        Self::new()
            .host(&config.host)
            .port(config.port)
            .username(&config.username)
            .password(&config.password)
            .database(&config.database)
    }
}

async fn health_check() -> StatusCode {
    StatusCode::OK
}

async fn health_check_db(State(pool): State<PgPool>) -> StatusCode {
    match sqlx::query("SELECT 1 + 1").fetch_one(&pool).await {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = DatabaseConfig {
        host: "localhost".to_string(),
        port: 5432,
        username: "app".to_string(),
        password: "passwd".to_string(),
        database: "app".to_string(),
    };

    let pool = config.connect();

    let app = Router::new()
        .route("/health", get(health_check))
        .route("/health/db", get(health_check_db))
        .with_state(pool);

    let addr = SocketAddr::new(Ipv4Addr::LOCALHOST.into(), 8080);
    let listener = TcpListener::bind(&addr).await?;

    println!("Listening on {}", addr);

    axum::serve(listener, app).await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_health_check_works() {
        let status_code = health_check().await;
        assert_eq!(status_code, StatusCode::OK);
    }

    #[sqlx::test]
    async fn test_health_check_db_works(pool: PgPool) {
        let status_code = health_check_db(State(pool)).await;
        assert_eq!(status_code, StatusCode::OK);
    }
}
