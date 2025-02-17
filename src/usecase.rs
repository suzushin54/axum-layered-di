use async_trait::async_trait;
use shaku::{Interface, Component};
use std::sync::Arc;
use crate::repository_interfaces::UserRepository;

#[async_trait]
pub trait UserUseCase: Interface {
    async fn execute(&self) -> String;
}

#[derive(Component)]
#[shaku(interface = UserUseCase)]
pub struct UserUseCaseImpl {
    #[shaku(inject)]
    user_repository: Arc<dyn UserRepository>, // shakuでは依存性をArcでラップする必要がある
}

#[async_trait]
impl UserUseCase for UserUseCaseImpl {
    async fn execute(&self) -> String {
        let user_data = self.user_repository.find_user().await;
        format!("UseCase processed: {}", user_data)
    }
}

// NOTE: shakuがコンポーネントの生成を管理するため不要（説明のため残している）
// #[allow(dead_code)]
// impl UserUseCaseImpl {
//     fn new(repository: Arc<dyn UserRepository>) -> Self {
//         Self { user_repository: repository }
//     }
// } 