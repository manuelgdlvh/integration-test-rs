use std::sync::Arc;

use crate::repositories::user_repository::UserRepository;

pub trait Service {
    fn get(&self) -> Vec<String>;
}

pub type UserService = Arc<dyn Service + Send + Sync>;

pub struct UserServiceImpl {
    user_repository: UserRepository,
}

impl UserServiceImpl {
    pub fn new(user_repository: UserRepository) -> Self {
        Self { user_repository }
    }
}

impl Service for UserServiceImpl {
    fn get(&self) -> Vec<String> {
        self.user_repository.get()
    }
}