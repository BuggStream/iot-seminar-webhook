use axum::{
    routing::{get, post},
    http::StatusCode,
    Json, Router,
};
use serde::Deserialize;
use serde_json::Value;
use tracing::info;

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // build our application with a route
    let app = Router::new()
        .route("/", get(root))
        .route("/uplink", post(uplink))
        .route("/join", post(join))
        .route("/location", post(location));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    info!("Listening on http://0.0.0.0:3000");
    axum::serve(listener, app).await.unwrap();
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    info!("Received message at root");
    "Hello, World!"
}

async fn uplink(
    Json(payload): Json<Uplink>,
) -> StatusCode {
    let payload = payload.uplink_message.decoded_payload;
    info!("Decoded payload json:\n{}", payload.to_string());

    StatusCode::OK
}

async fn join(
    Json(payload): Json<Value>,
) -> StatusCode {
    info!("Join json:\n{}", payload.to_string());

    StatusCode::OK
}

async fn location(
    Json(payload): Json<Value>,
) -> StatusCode {
    info!("Location json:\n{}", payload.to_string());

    StatusCode::OK
}

#[derive(Deserialize)]
struct Uplink {
    uplink_message: UplinkMessage,
}

#[derive(Deserialize)]
struct UplinkMessage {
    decoded_payload: Value,
}


