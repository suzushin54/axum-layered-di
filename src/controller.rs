use axum::response::IntoResponse;
use shaku::HasComponent;
use std::sync::Arc;

use crate::usecase::UserUseCase;

pub async fn user_handler<M>(container: Arc<M>) -> impl IntoResponse 
where
    M: HasComponent<dyn UserUseCase>
{
    // DIコンテナからUseCaseを取得
    let usecase: &dyn UserUseCase = container.resolve_ref();
    
    // UseCaseの実行
    let result = usecase.execute().await;
    result
} 