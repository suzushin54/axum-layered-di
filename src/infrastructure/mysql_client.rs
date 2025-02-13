use shaku::{Component, Interface};
use sqlx::MySqlPool;
use std::sync::Arc;

pub trait IMysqlClient: Interface {
    fn get_pool(&self) -> Arc<MySqlPool>;
}

#[derive(Component)]
#[shaku(interface = IMysqlClient)]
pub struct MysqlClientImpl {
    pool: Arc<MySqlPool>,
}

impl MysqlClientImpl {
    pub fn new(pool: Arc<MySqlPool>) -> Self {
        Self { pool }
    }
}

impl IMysqlClient for MysqlClientImpl {
    fn get_pool(&self) -> Arc<MySqlPool> {
        self.pool.clone()
    }
} 