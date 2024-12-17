use csv::Writer;
use dotenvy::dotenv;
use futures::TryStreamExt;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};
use sqlx::types::chrono::{DateTime, Local, NaiveDateTime, TimeZone};
use sqlx::{ConnectOptions, PgPool};
use std::error::Error;
use std::path::PathBuf;
use std::str::FromStr;
use std::time::Duration;
use std::{env, fs};
use tracing::log::LevelFilter;

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
struct Uplink {
    uplink_message: UplinkMessage,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
struct UplinkMessage {
    decoded_payload: LoraPayload,
    rx_metadata: Vec<Option<Gateway>>,
}

#[derive(Deserialize, Debug)]
struct LoraPayload {
    latitude: f64,
    longitude: f64,
    #[serde(alias = "gpsUsedSats")]
    gps_satellites: u16,
    #[serde(alias = "rxCount")]
    rx_count: u64,
}

#[derive(Deserialize, Debug, Clone)]
struct Gateway {
    gateway_ids: GatewayId,
    received_at: Option<DateTime<Local>>,
    rssi: i64,
    snr: Option<f64>,
    location: Option<Location>,
}

#[derive(Deserialize, Debug, Clone)]
struct Location {
    latitude: f64,
    longitude: f64,
    altitude: Option<f64>,
}

#[derive(Deserialize, Debug, Clone)]
struct GatewayId {
    eui: Option<String>,
    gateway_id: Option<String>,
}

#[derive(Serialize)]
struct CsvRow {
    message_id: i64,
    gateway_id: String,
    received_at: DateTime<Local>,
    rx_lat: f64,
    rx_lng: f64,
    rssi: i64,
    snr: f64,
    rx_count: u64,
    satellites_used: u16,
    gps_lat: f64,
    gps_lng: f64,
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

    let mut path_buf = PathBuf::new();
    path_buf.push("datasets");
    fs::create_dir_all(path_buf.as_path())?;
    path_buf.push(&format!("{}__{}.csv", start_naive, end_naive));
    let mut writer = Writer::from_path(path_buf.as_path())?;

    for (id, message) in messages {
        for gateway in message.uplink_message.rx_metadata.iter().filter_map(|x| x.clone()) {
            if gateway.location.is_none() {
                continue;
            }

            let gateway_id = gateway.gateway_ids.gateway_id.unwrap_or(String::from("anonymous"));
            let location = gateway.location.unwrap();
            let row = CsvRow {
                message_id: id,
                gateway_id,
                received_at: gateway.received_at.unwrap(),
                rx_lat: location.latitude,
                rx_lng: location.longitude,
                rssi: gateway.rssi,
                snr: gateway.snr.unwrap_or(f64::NAN),
                rx_count: message.uplink_message.decoded_payload.rx_count,
                satellites_used: message.uplink_message.decoded_payload.gps_satellites,
                gps_lat: message.uplink_message.decoded_payload.latitude,
                gps_lng: message.uplink_message.decoded_payload.longitude,
            };

            writer.serialize(row)?;
        }

        println!("{:#?}", message.uplink_message.decoded_payload);
    }

    writer.flush()?;

    Ok(())
}

pub async fn dataset(
    pool: &PgPool,
    start: DateTime<Local>,
    end: DateTime<Local>,
) -> Result<Vec<(i64, Uplink)>, Box<dyn Error>> {
    let mut stream = sqlx::query_as::<_, (i64, DateTime<Local>, Value)>(
        r#"SELECT * FROM uplink WHERE "timestamp" > $1 AND "timestamp" < $2"#,
    )
    .bind(start)
    .bind(end)
    .fetch(pool);

    let mut messages = Vec::new();

    while let Some((id, _, message)) = stream.try_next().await? {
        if let Ok(uplink) = serde_json::from_value::<Uplink>(message) {
            messages.push((id, uplink));
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
