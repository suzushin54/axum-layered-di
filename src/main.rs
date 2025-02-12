mod controller;
mod di;
mod router;
mod usecase;

use crate::di::create_container;
use crate::router::create_router;

#[tokio::main]
async fn main() {
    // DIコンテナの作成
    let container = create_container();
    
    // ルーターの作成（DIコンテナを渡す）
    let app = create_router(container);
    
    // サーバーの起動
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("Server running on http://127.0.0.1:3000");
    axum::serve(listener, app).await.unwrap();
}
