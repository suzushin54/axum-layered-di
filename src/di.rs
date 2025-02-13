use shaku::module;
use std::sync::Arc;

use crate::usecase::UserUseCaseImpl;
use crate::infrastructure::repositories::user_repository::UserRepositoryImpl;
use crate::infrastructure::mysql_client::{MysqlClientImpl, MysqlClientImplParameters};
use crate::config::Config;
use sqlx::MySqlPool;

module! {
    pub DIContainer {
        components = [UserUseCaseImpl, UserRepositoryImpl, MysqlClientImpl],
        providers = []
    }
}

pub async fn create_container() -> anyhow::Result<Arc<DIContainer>> {
    let config = Config::from_env();
    let pool = MySqlPool::connect(&config.database_url).await?;
    let pool = Arc::new(pool);
    
    let builder = DIContainer::builder();
    let container = builder
        .with_component_parameters::<MysqlClientImpl>(MysqlClientImplParameters { pool })
        .build();
    
    Ok(Arc::new(container))
} 