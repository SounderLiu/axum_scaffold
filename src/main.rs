



use axum::{
    extract::{Path, Query},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    // Initialize tracing subscriber for logging
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "my_axum_app=debug,tower_http=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Build the application with routes
    let app = Router::new()
        .route("/", get(root))
        .route("/hello/:name", get(hello))
        .route("/calculate", get(calculate))
        .route("/echo", post(echo))
        .layer(tower_http::trace::TraceLayer::new_for_http());

    // Define the address to run the server on
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    tracing::info!("Listening on {}", addr);

    // Run the server
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// Handler for the root route "/"
async fn root() -> &'static str {
    "Welcome to my Axum app!"
}

// Handler for the "/hello/:name" route
async fn hello(Path(name): Path<String>) -> impl IntoResponse {
    format!("Hello, {}!", name)
}

// Handler for the "/calculate" route to handle query parameters
async fn calculate(Query(params): Query<CalcParams>) -> impl IntoResponse {
    let result = params.a + params.b;
    Json(CalcResult { result })
}

#[derive(Deserialize)]
struct CalcParams {
    a: i32,
    b: i32,
}

#[derive(Serialize)]
struct CalcResult {
    result: i32,
}

// Handler for the "/echo" route to handle JSON POST requests
async fn echo(Json(payload): Json<EchoRequest>) -> impl IntoResponse {
    let response = EchoResponse {
        message: payload.message,
    };
    (StatusCode::OK, Json(response))
}

#[derive(Deserialize)]
struct EchoRequest {
    message: String,
}

#[derive(Serialize)]
struct EchoResponse {
    message: String,
}
