use serde::Serialize;

use crate::handlers::requests::create_user_request::CreateUserRequest;

#[derive(Serialize)]
pub struct CreateUserResponse<'a> {
    response: &'a str,
}

impl<'a> CreateUserResponse<'a> {
    pub fn new(response: &'a str) -> Self {
        Self { response }
    }

    pub fn map(from: &'a CreateUserRequest) -> Self {
        Self {
            response: from.data(),
        }
    }
}



