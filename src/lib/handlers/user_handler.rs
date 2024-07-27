use axum::http::StatusCode;
use axum::Json;
use axum::response::{IntoResponse, Response};
use axum_extra::extract::JsonDeserializer;

use crate::handlers::requests::create_user_request::CreateUserRequest;
use crate::handlers::responses::create_user_response::CreateUserResponse;

macro_rules! deserialize_or_bail {
    ($payload:expr) => {
        match $payload.deserialize() {
            Ok(data) => data,
            Err(e) => return e.into_response(),
        }
    };
}

pub async fn create_user(
    payload: JsonDeserializer<CreateUserRequest<'_>>,
) -> Response {
    let input = deserialize_or_bail!(payload);
    let response = CreateUserResponse::map(&input);
    (StatusCode::CREATED, Json(response)).into_response()
}