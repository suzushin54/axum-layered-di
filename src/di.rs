use shaku::module;
use std::sync::Arc;

use crate::usecase::UserUseCaseImpl;
use crate::infrastructure::repositories::user_repository::UserRepositoryImpl;

// DIモジュールの定義
module! {
    pub DIContainer {
        components = [UserUseCaseImpl, UserRepositoryImpl],
        providers = []
    }
}

// DIコンテナのインスタンスを作成する関数
pub fn create_container() -> Arc<DIContainer> {
    let builder = DIContainer::builder();
    Arc::new(builder.build())
} 