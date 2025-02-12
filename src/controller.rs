use axum::{
    response::IntoResponse,
    extract::State,
};
use shaku::HasComponent;
use crate::app_state::AppState;

pub async fn user_handler(
    State(state): State<AppState>,
) -> impl IntoResponse {
    let usecase = state.container.resolve_ref();
    let result = usecase.execute().await;
    result
} 