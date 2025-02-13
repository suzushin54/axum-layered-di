use async_trait::async_trait;
use shaku::Component;
use std::sync::Arc;
use crate::repository_interfaces::UserRepository;
use crate::infrastructure::mysql_client::IMysqlClient;

#[derive(Component)]
#[shaku(interface = UserRepository)]
pub struct UserRepositoryImpl {
    #[shaku(inject)]
    mysql_client: Arc<dyn IMysqlClient>,
}

impl UserRepositoryImpl {
    pub fn new(mysql_client: Arc<dyn IMysqlClient>) -> Self {
        Self { mysql_client }
    }
}

#[async_trait]
impl UserRepository for UserRepositoryImpl {
    async fn find_user(&self) -> String {
        let pool = self.mysql_client.get_pool();
        // ここでpoolを使用してデータベースアクセスを実装
        "example user data from UserRepositoryImpl".to_string()
    }
}
