use async_trait::async_trait;
use shaku::Interface;
use anyhow::Result;

#[async_trait]
pub trait UserRepository: Interface {
    async fn find_user(&self) -> String;
    async fn save(&self, name: String, description: Option<String>, price: i32) -> Result<()>;
} 