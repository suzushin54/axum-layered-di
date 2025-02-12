use std::sync::Arc;
use crate::di::DIContainer;

#[derive(Clone)]
pub struct AppState {
    pub container: Arc<DIContainer>,
} 