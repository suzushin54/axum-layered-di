use async_trait::async_trait;
use shaku::Component;
use std::sync::Arc;
use sqlx::FromRow;
use chrono::{DateTime, Utc};
use crate::repository_interfaces::UserRepository;
use crate::infrastructure::mysql_client::IMysqlClient;

#[derive(FromRow)]
struct ProductRow {
    id: String,
    name: String,
    description: Option<String>,
    price: i32,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

#[derive(Component)]
#[shaku(interface = UserRepository)]
pub struct UserRepositoryImpl {
    #[shaku(inject)]
    mysql_client: Arc<dyn IMysqlClient>,
}

impl UserRepositoryImpl {
    pub fn new(mysql_client: Arc<dyn IMysqlClient>) -> Self {
        Self { mysql_client }
    }
}

#[async_trait]
impl UserRepository for UserRepositoryImpl {
    async fn find_user(&self) -> String {
        let pool = self.mysql_client.get_pool();
        
        let products = sqlx::query_as::<_, ProductRow>(
            "SELECT id, name, description, price, created_at, updated_at FROM products LIMIT 5"
        )
        .fetch_all(&*pool)
        .await
        .unwrap_or_default();

        let result = products
            .into_iter()
            .map(|p| format!(
                "id: {}\nname: {}\ndescription: {}\nprice: {}\ncreated_at: {}\nupdated_at: {}\n",
                p.id,
                p.name,
                p.description.unwrap_or_else(|| "No description".to_string()),
                p.price,
                p.created_at,
                p.updated_at
            ))
            .collect::<Vec<_>>()
            .join("\n");

        if result.is_empty() {
            "No products found".to_string()
        } else {
            result
        }
    }
}
