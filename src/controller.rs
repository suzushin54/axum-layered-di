use axum::{
    response::IntoResponse,
    extract::State,
};
use shaku::HasComponent;
use crate::app_state::AppState;
use crate::usecase::UserUseCase;

pub async fn user_handler(
    State(state): State<AppState>,
) -> impl IntoResponse {
    let usecase: &dyn UserUseCase = state.container.resolve_ref();
    let result = usecase.execute().await;
    result
} 