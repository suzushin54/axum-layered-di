use async_trait::async_trait;
use shaku::Interface;

#[async_trait]
pub trait UserRepository: Interface {
    async fn find_user(&self) -> String;
} 