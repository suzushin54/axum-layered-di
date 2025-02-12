use axum::{
    routing::get,
    Router,
};
use crate::controller::user_handler;
use crate::app_state::AppState;

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/users", get(user_handler))
        .with_state(state)
} 