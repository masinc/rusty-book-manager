use adapter::database::ConnectionPool;
use adapter::repository::health::HealthCheckRepositoryImpl;
use kernel::repository::health::HealthCheckRepository;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppRegistry {
    health_check_repository: Arc<dyn HealthCheckRepository>,
}

impl AppRegistry {
    pub fn new(pool: ConnectionPool) -> Self {
        let health_check_repository = Arc::new(HealthCheckRepositoryImpl::new(pool));
        Self {
            health_check_repository,
        }
    }

    pub fn health_check_repository(&self) -> Arc<dyn HealthCheckRepository> {
        self.health_check_repository.clone()
    }
}
