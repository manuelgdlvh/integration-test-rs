use std::borrow::Cow;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateUserRequest<'a> {
    #[serde(borrow)]
    data: Cow<'a, str>,
}

impl<'a> CreateUserRequest<'a> {
    pub fn new(data: Cow<'a, str>) -> Self {
        Self { data }
    }

    pub fn data(&self) -> &Cow<'a, str> {
        &self.data
    }
}

