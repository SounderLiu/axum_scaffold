use axum::{extract::Path, response::IntoResponse, Json, Router};
use axum::routing::get;
use crate::models::user::User;

pub fn routes() -> Router {
    Router::new().route("get", get(get_user))
}

pub async fn get_user(Path(user_id): Path<u32>) -> impl IntoResponse {
    // 您可以从数据库或其他源获取用户信息
    let user = User {
        id: user_id,
        name: "Alice".to_string(),
        email: "alice@mail.com".to_string()
    };
    Json(user)
}