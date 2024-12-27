use axum::extract::State;
use axum::http::StatusCode;
use registry::AppRegistry;

pub async fn health_check() -> StatusCode {
    StatusCode::OK
}

pub async fn health_check_db(State(registry): State<AppRegistry>) -> StatusCode {
    if registry.health_check_repository().check_db().await {
        StatusCode::OK
    } else {
        StatusCode::INTERNAL_SERVER_ERROR
    }
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
