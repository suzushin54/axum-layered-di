use shaku::module;
use std::sync::Arc;

use crate::usecase::UserUseCaseImpl;

// DIモジュールの定義
module! {
    pub DIContainer {
        components = [UserUseCaseImpl],
        providers = []
    }
}

// DIコンテナのインスタンスを作成する関数
pub fn create_container() -> Arc<DIContainer> {
    let builder = DIContainer::builder();
    Arc::new(builder.build())
} 