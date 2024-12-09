use axum::extract::State;
use axum::{
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use dotenv::dotenv;
use serde::Deserialize;
use serde_json::{json, Value};
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::env;
use std::time::Duration;
use tokio::net::TcpListener;
use tracing::info;

#[tokio::main]
async fn main() {
    dotenv().ok();
    // initialize tracing
    tracing_subscriber::fmt::init();

    let port = env::var("PORT").unwrap_or("3000".to_string());

    let db_url = env::var("DATABASE_URL").expect("No database URL variable configured!");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&db_url)
        .await
        .expect("can't connect to database");

    // build our application with a route
    let app = Router::new()
        .route("/", get(root))
        .route("/uplink", post(uplink))
        .route("/join", post(join))
        .route("/location", post(location))
        .with_state(pool);

    let url = format!("0.0.0.0:{}", port);

    // run our app with hyper, listening globally on port 3000
    let listener = TcpListener::bind(&url).await.unwrap();
    info!("Listening on {}", &url);
    axum::serve(listener, app).await.unwrap();
}

// basic handler that responds with a static string
async fn root(State(pool): State<PgPool>) -> Result<&'static str, (StatusCode, String)> {
    info!("Received message at root");

    sqlx::query("INSERT INTO public.uplink(message) VALUES ($1)")
        .bind(json!({"id": 5, "name": "test"}))
        .execute(&pool)
        .await
        .map_err(database_error)?;

    let results = sqlx::query("SELECT * FROM uplink")
        .fetch_all(&pool)
        .await
        .map_err(database_error)?;

    info!("Results: {}", results.len());

    Ok("Hello, World!")
}

async fn uplink(Json(payload): Json<Uplink>) -> StatusCode {
    let decoded = &payload.uplink_message.decoded_payload;
    info!("Decoded payload json:\n{}", decoded);
    info!("Entire payload json:\n{:?}", payload);

    StatusCode::OK
}

async fn join(Json(payload): Json<Value>) -> StatusCode {
    info!("Join json:\n{}", payload.to_string());

    StatusCode::OK
}

async fn location(Json(payload): Json<Value>) -> StatusCode {
    info!("Location json:\n{}", payload.to_string());

    StatusCode::OK
}

#[derive(Deserialize, Debug)]
struct Uplink {
    uplink_message: UplinkMessage,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
struct UplinkMessage {
    decoded_payload: Value,
    rx_metadata: Value,
}

fn database_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}
