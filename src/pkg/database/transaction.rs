use sqlx::{Database, Pool, Transaction};
use anyhow::Result;
use std::future::Future;

/// Executes a database transaction, ensuring atomicity.
/// Runs the given operation inside a transaction, committing on success or rolling back on failure.
/// Uses `FnOnce` to allow ownership moves and ensure the operation runs only once.
/// 
/// # Arguments
/// 
/// * `pool` - The database pool.
/// * `operation` - The operation to run within the transaction.
/// 
/// # Returns
pub async fn run_transaction<DB, F, Fut, T>(
    pool: &Pool<DB>,
    operation: F,
) -> Result<T>
where
    DB: Database,
    F: FnOnce(&mut Transaction<'_, DB>) -> Fut,
    Fut: Future<Output = Result<T>>,
{
    let mut tx = pool.begin().await?;

    match operation(&mut tx).await {
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
