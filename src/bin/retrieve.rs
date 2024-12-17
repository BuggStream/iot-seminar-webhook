use chrono::Utc;
use dotenvy::dotenv;
use futures::{StreamExt, TryStreamExt};
use serde::Deserialize;
use serde_json::Value;
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};
use sqlx::types::chrono::{DateTime, Local, NaiveDateTime, TimeZone};
use sqlx::{ConnectOptions, PgPool};
use std::env;
use std::error::Error;
use std::str::FromStr;
use std::time::Duration;
use tracing::log::LevelFilter;

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
struct Uplink {
    uplink_message: UplinkMessage,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
struct UplinkMessage {
    decoded_payload: Value,
    rx_metadata: Vec<Option<Gateway>>,
}

#[derive(Deserialize, Debug)]
struct Gateway {
    gateway_ids: GatewayId,
    received_at: Option<DateTime<Local>>,
    rssi: i64,
    snr: Option<f64>,
    location: Option<Location>,
}

#[derive(Deserialize, Debug)]
struct Location {
    latitude: f64,
    longitude: f64,
    altitude: Option<f64>,
}

#[derive(Deserialize, Debug)]
struct GatewayId {
    eui: Option<String>,
    gateway_id: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    // initialize tracing
    tracing_subscriber::fmt::init();

    let start_naive = NaiveDateTime::parse_from_str("2024-12-13 11:00:00", "%Y-%m-%d %H:%M:%S")?;
    let end_naive = NaiveDateTime::parse_from_str("2024-12-13 18:00:00", "%Y-%m-%d %H:%M:%S")?;

    let start = Local.from_local_datetime(&start_naive).single().unwrap();
    let end = Local.from_local_datetime(&end_naive).single().unwrap();

    println!("{}", start);

    let pool = connect_database().await;
    let messages = dataset(&pool, start, end).await?;

    for message in messages {
        println!("{:#?}", message);
    }

    Ok(())
}

pub async fn dataset(
    pool: &PgPool,
    start: DateTime<Local>,
    end: DateTime<Local>,
) -> Result<Vec<Uplink>, Box<dyn Error>> {
    let mut stream = sqlx::query_as::<_, (i64, DateTime<Local>, Value)>(
        r#"SELECT * FROM uplink WHERE "timestamp" > $1 AND "timestamp" < $2"#,
    )
    .bind(start)
    .bind(end)
    .fetch(pool);

    let mut messages = Vec::new();

    while let Some((_, _, message)) = stream.try_next().await? {
        // let json_gateways = &message["data"]["uplink_message"]["rx_metadata"];
        // let gateways: Vec<Gateway> = serde_json::from_value(json_gateways.clone())?;
        // println!("{:?}", &gateways);

        if let Ok(uplink) = serde_json::from_value::<Uplink>(message) {
            messages.push(uplink);
        };
    }

    Ok(messages)
}

async fn connect_database() -> PgPool {
    let db_url = env::var("DATABASE_REMOTE_URL").expect("No database URL variable configured!");

    let opts = PgConnectOptions::from_str(&db_url)
        .unwrap()
        .log_statements(LevelFilter::Info);

    PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect_with(opts)
        .await
        .expect("can't connect to database")
}
