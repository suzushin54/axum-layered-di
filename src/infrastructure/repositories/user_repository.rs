use async_trait::async_trait;
use shaku::Component;
use crate::repository_interfaces::UserRepository;

#[derive(Component)]
#[shaku(interface = UserRepository)]
pub struct UserRepositoryImpl;

#[async_trait]
impl UserRepository for UserRepositoryImpl {
    async fn find_user(&self) -> String {
        "example user data from UserRepositoryImpl".to_string()
    }
}
