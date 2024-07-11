use serde::Serialize;

#[derive(Serialize)]
pub struct CreateUserResponse {
    data: Vec<String>,
}

impl CreateUserResponse {
    pub fn new(data: Vec<String>) -> Self {
        Self { data }
    }
}