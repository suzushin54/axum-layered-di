use chrono::{DateTime, Utc};

pub struct Product {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub price: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
} 