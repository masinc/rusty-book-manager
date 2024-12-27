use adapter::database::connect_database_with;
use api::route::book::build_book_routes;
use api::route::health::build_health_check_routers;
use axum::Router;
use registry::AppRegistry;
use shared::config::AppConfig;
use std::net::{Ipv4Addr, SocketAddr};
use tokio::net::TcpListener;

async fn bootstrap() -> anyhow::Result<()> {
    let app_config = AppConfig::new()?;
    let pool = connect_database_with(&app_config.database);

    let registry = AppRegistry::new(pool);

    let app = Router::new()
        .merge(build_book_routes())
        .merge(build_health_check_routers())
        .with_state(registry);

    let addr = SocketAddr::new(Ipv4Addr::LOCALHOST.into(), 8080);
    let listener = TcpListener::bind(&addr).await?;

    println!("Server is running on {}", addr);

    axum::serve(listener, app.into_make_service())
        .await
        .map_err(anyhow::Error::from)
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    bootstrap().await
}
