use axum::{
    routing::get,
    Router,
};
use crate::handlers::home_handler;

pub fn routes() -> Router {
    Router::new().route("/", get(home_handler::index))
}