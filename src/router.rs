use axum::{
    routing::{get, post},
    Router,
};
use crate::controller::{user_handler, create_product_handler};
use crate::app_state::AppState;

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/users", get(user_handler))
        .route("/products", post(create_product_handler))
        .with_state(state)
} 