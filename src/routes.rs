use std::sync::Arc;

use axum::{
    extract::{Json, State},
    routing::get,
    Router,
};

use serde::Deserialize;

use crate::{
    prisma::{self, user::Data},
    AppState, error::AppError,
};
type AppResult<T> = Result<T, AppError>;
type AppJsonResult<T> = AppResult<Json<T>>;

#[derive(Debug, Deserialize)]
struct UserRequest {
    display_name: String,
}

pub async fn create_route() -> Router<AppState> {
    let client = Arc::new(prisma::new_client().await.unwrap());
    Router::with_state(AppState { client })
        .route("/user", get(handle_get_users).post(handle_create_user))
}

async fn handle_create_user(
    State(state): State<AppState>,
    Json(input): Json<UserRequest>,
) -> AppJsonResult<Data> {
    let user = state
        .client
        .user()
        .create(input.display_name, vec![])
        .exec()
        .await?;
    Ok(Json::from(user))
}

async fn handle_get_users(State(state): State<AppState>) -> AppJsonResult<Vec<Data>> {
    let users = state.client.user().find_many(vec![]).exec().await?;
    Ok(Json::from(users))
}

