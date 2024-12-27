pub mod home;
pub mod users;
pub mod api;
mod home;

use axum::Router;

pub fn create_routes() -> Router {
    Router::new()
        .merge(home::routes())
        .nest("/users", users::routes())
        .nest("/api", api::routes())
}