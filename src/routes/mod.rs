mod home;
mod user;

use axum::Router;

pub fn create_routes() -> Router {
    Router::new()
        .merge(home::routes())
        .nest("/users", user::routes())
        // .nest("/api", api::routes())
}