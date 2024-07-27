use std::sync::Arc;

use mockall::{lazy_static, mock};

use lib::repositories::user_repository::Repository;
use lib::services::user_service::{UserService, UserServiceImpl};

lazy_static! {
        static ref TEST_DATA: Vec<String> = vec!["0".to_string(), "1".to_string()];
}

mock! {
    pub UserRepository {}
    impl Repository for UserRepository {
        fn get(&self) -> Vec<String>;
    }
}

fn init(user_repository: MockUserRepository) -> UserService {
    let user_repository_mock = Arc::new(user_repository);
    let user_service: UserService = Arc::new(UserServiceImpl::new(user_repository_mock));
    user_service
}

#[test]
fn should_returns_data() {
    // When
    let mut user_repository_mock = MockUserRepository::new();
    user_repository_mock.expect_get().returning(|| { TEST_DATA.to_vec() });

    // Then
    let user_service = init(user_repository_mock);
    assert_eq!(TEST_DATA.to_vec(), user_service.get());
}

#[test]
fn should_not_match_data_returned() {
    // When
    let mut user_repository_mock = MockUserRepository::new();
    user_repository_mock.expect_get().returning(|| { TEST_DATA.to_vec() });

    // Then
    let user_service = init(user_repository_mock);
    assert_ne!(vec!["0".to_string()], user_service.get());
}