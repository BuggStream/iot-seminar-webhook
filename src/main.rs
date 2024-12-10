mod queries;

use crate::queries::{store_join, store_location, store_uplink, uplink_count};
use axum::extract::State;
use axum::{
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use dotenvy::dotenv;
use serde::Deserialize;
use serde_json::Value;
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
async fn root(State(pool): State<PgPool>) -> Result<String, (StatusCode, String)> {
    let count = uplink_count(&pool).await?;
    let message = format!("Uplink count: {}", count);
    info!("{}", &message);

    Ok(message)
}

async fn uplink(
    State(pool): State<PgPool>,
    Json(payload): Json<Value>,
) -> Result<StatusCode, (StatusCode, String)> {
    info!("Uplink payload json:\n{:?}", payload);
    store_uplink(&pool, payload).await?;

    Ok(StatusCode::OK)
}

async fn join(
    State(pool): State<PgPool>,
    Json(payload): Json<Value>,
) -> Result<StatusCode, (StatusCode, String)> {
    info!("Join payload json:\n{:?}", payload);
    store_join(&pool, payload).await?;

    Ok(StatusCode::OK)
}

async fn location(
    State(pool): State<PgPool>,
    Json(payload): Json<Value>,
) -> Result<StatusCode, (StatusCode, String)> {
    info!("Location payload json:\n{:?}", payload);
    store_location(&pool, payload).await?;

    Ok(StatusCode::OK)
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
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
