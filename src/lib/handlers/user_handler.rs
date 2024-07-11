use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;

use crate::handlers::response::create_user_response::CreateUserResponse;
use crate::services::user_service::UserService;

pub async fn create_user(
    State(state): State<UserService>
) -> (StatusCode, Json<CreateUserResponse>) {
    let response = CreateUserResponse::new(state.get());
    (StatusCode::CREATED, Json(response))
}