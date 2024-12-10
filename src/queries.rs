use crate::database_error;
use axum::http::StatusCode;
use serde_json::Value;
use sqlx::postgres::PgQueryResult;
use sqlx::PgPool;
use sqlx::types::chrono::Local;

pub async fn store_uplink(
    pool: &PgPool,
    message: Value,
) -> Result<PgQueryResult, (StatusCode, String)> {
    let timestamp = Local::now();

    sqlx::query("INSERT INTO public.uplink(timestamp, message) VALUES ($1, $2)")
        .bind(timestamp)
        .bind(message)
        .execute(pool)
        .await
        .map_err(database_error)
}

pub async fn uplink_count(pool: &PgPool) -> Result<i64, (StatusCode, String)> {
    let (num,): (i64,) = sqlx::query_as(r#"SELECT count(*) as "count!" FROM uplink"#)
        .fetch_one(pool)
        .await
        .map_err(database_error)?;

    Ok(num)
}
