mod repository_interfaces;
mod infrastructure;
mod usecase;
mod controller;
mod router;
mod app_state;
mod di;
mod config;

use crate::di::create_container;
use crate::router::create_router;
use crate::app_state::AppState;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let container = create_container().await?;
    let state = AppState { container };
    
    let app = create_router(state);
    
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("Server running on http://127.0.0.1:3000");
    axum::serve(listener, app).await.unwrap();
    Ok(())
}
