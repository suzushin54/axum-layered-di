use async_trait::async_trait;
use shaku::{Interface, Component};

// UseCaseのインターフェース
#[async_trait]
pub trait UserUseCase: Interface {
    async fn execute(&self) -> String;
}

// UseCaseの実装
#[derive(Component)]
#[shaku(interface = UserUseCase)]
//#[shaku::interface(UserUseCase)]
pub struct UserUseCaseImpl;

#[async_trait]
impl UserUseCase for UserUseCaseImpl {
    async fn execute(&self) -> String {
        "Hello from UserUseCase!".to_string()
    }
}

// NOTE: shakuがコンポーネントの生成を管理するため不要（説明のため残している）
#[allow(dead_code)]
impl UserUseCaseImpl {
    fn new() -> Self {
        Self
    }
} 