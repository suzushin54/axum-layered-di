use sqlx::{MySql, Transaction};
use std::sync::Arc;
use anyhow::Result;
use std::future::Future;

pub async fn transaction<F, Fut, T>(
    pool: &Arc<sqlx::MySqlPool>,
    f: F,
) -> Result<T>
where
    F: FnOnce(&mut Transaction<'_, MySql>) -> Fut,
    Fut: Future<Output = Result<T>>,
{
    let mut tx = pool.begin().await?;
    
    match f(&mut tx).await {
        Ok(result) => {
            tx.commit().await?;
            Ok(result)
        }
        Err(e) => {
            tx.rollback().await?;
            Err(e)
        }
    }
} 