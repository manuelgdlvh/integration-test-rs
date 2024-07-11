use std::sync::Arc;
use sqlx::{Pool, Postgres};

pub trait Repository {
    fn get(&self) -> Vec<String>;
}

pub type UserRepository = Arc<dyn Repository + Send + Sync>;

pub struct UserRepositoryImpl {
    db_pool: Pool<Postgres>,
}

impl UserRepositoryImpl {
    pub fn new(db_pool: Pool<Postgres>) -> Self {
        Self { db_pool }
    }
}

impl Repository for UserRepositoryImpl {
    fn get(&self) -> Vec<String> {
        vec!["0".to_string(), "1".to_string()]
    }
}

