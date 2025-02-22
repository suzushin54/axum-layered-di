use async_trait::async_trait;
use shaku::{Interface, Component};
use std::sync::Arc;
use anyhow::Result;
use crate::repository_interfaces::UserRepository;
use crate::pkg::database::mysql::IMysqlClient;
use crate::pkg::database::transaction;

#[async_trait]
pub trait UserUseCase: Interface {
    async fn execute(&self) -> String;
    async fn create_product(&self, name: String, description: Option<String>, price: i32) -> Result<()>;
}

#[derive(Component)]
#[shaku(interface = UserUseCase)]
pub struct UserUseCaseImpl {
    #[shaku(inject)]
    user_repository: Arc<dyn UserRepository>, // shakuでは依存性をArcでラップする必要がある
    #[shaku(inject)]
    mysql_client: Arc<dyn IMysqlClient>,
}

#[async_trait]
impl UserUseCase for UserUseCaseImpl {
    async fn execute(&self) -> String {
        let user_data = self.user_repository.find_user().await;
        format!("UseCase processed: {}", user_data)
    }

    async fn create_product(&self, name: String, description: Option<String>, price: i32) -> Result<()> {
        let pool = self.mysql_client.get_pool();
        
        transaction(&pool, |_tx| async move {
            self.user_repository.save(name, description, price).await?;
            Ok(())
        })
        .await
    }
}

// NOTE: shakuがコンポーネントの生成を管理するため不要（説明のため残している）
// #[allow(dead_code)]
// impl UserUseCaseImpl {
//     fn new(repository: Arc<dyn UserRepository>) -> Self {
//         Self { user_repository: repository }
//     }
// } 