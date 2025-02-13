use axum::{
    response::IntoResponse,
    extract::State,
};
use crate::app_state::AppState;
use crate::usecase::UserUseCase;
use shaku::HasComponent;
use std::sync::Arc;

pub async fn user_handler(
    State(state): State<AppState>,
) -> impl IntoResponse {
    let usecase: Arc<dyn UserUseCase> = state.container.resolve();
    let result = usecase.execute().await;
    result
} 