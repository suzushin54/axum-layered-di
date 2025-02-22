use axum::{
    response::IntoResponse,
    extract::{State, Json},
    http::StatusCode,
};
use crate::app_state::AppState;
use crate::usecase::UserUseCase;
use shaku::HasComponent;
use std::sync::Arc;
use serde::Deserialize;

pub async fn user_handler(
    State(state): State<AppState>,
) -> impl IntoResponse {
    let usecase: Arc<dyn UserUseCase> = state.container.resolve();
    let result = usecase.execute().await;
    result
}

#[derive(Deserialize)]
pub struct CreateProductRequest {
    name: String,
    description: Option<String>,
    price: i32,
}

pub async fn create_product_handler(
    State(state): State<AppState>,
    Json(payload): Json<CreateProductRequest>,
) -> impl IntoResponse {
    let usecase: Arc<dyn UserUseCase> = state.container.resolve();
    
    match usecase.create_product(payload.name, payload.description, payload.price).await {
        Ok(_) => StatusCode::CREATED,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
} 