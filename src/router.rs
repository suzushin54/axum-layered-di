use axum::{
    routing::get,
    Router,
    extract::State,
};
use std::sync::Arc;
use shaku::HasComponent;
use crate::controller::user_handler;
use crate::usecase::UserUseCase;

pub fn create_router<M>(container: Arc<M>) -> Router 
where
    M: HasComponent<dyn UserUseCase> + Send + Sync + 'static
{
    Router::new()
        .route("/users", get(|State(container): State<Arc<M>>| user_handler(container)))
        .with_state(container)
} 