use axum::{
    body::Bytes,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};

use crate::device::{Device, DeviceData, DeviceSchema};

use axum::extract::State;
use chrono::{DateTime, NaiveDateTime, Utc};
use deadpool_postgres::{GenericClient, Pool};
use uuid::Uuid;

// (Placeholder) Returns the status of the application.
pub async fn health() -> impl IntoResponse {
    (StatusCode::OK, "OK")
}

/// Returns a list of connected devices.
pub async fn get_devices(State(pool): State<Pool>) -> Result<Response, (StatusCode, String)> {
    let conn = pool.get().await.map_err(internal_error)?;
    let stmt = conn
        .prepare_cached("SELECT id, uuid, mac, firmware, created_at, updated_at FROM axum_device;")
        .await
        .map_err(internal_error)?;

    let res = conn.query(&stmt, &[]).await.map_err(internal_error)?;

    let mut users: Vec<DeviceSchema> = vec![];

    for row in res {
        let id: Uuid = row.get("uuid");
        let mac: String = row.get("mac");
        let firmware: String = row.get("firmware");
        let created_at: NaiveDateTime = row.get("created_at");
        let updated_at: NaiveDateTime = row.get("updated_at");
        users.push(DeviceSchema {
            id,
            mac,
            firmware,
            created_at: created_at.and_utc(),
            updated_at: updated_at.and_utc(),
        });
    }

    Ok(Json(users).into_response())
}

pub async fn create_device(
    State(pool): State<Pool>,
    bytes: Bytes,
) -> Result<Response, (StatusCode, String)> {
    let conn = pool.get().await.map_err(internal_error)?;

    let device: DeviceData = serde_json::from_slice(&bytes).map_err(|err| {
        (
            StatusCode::BAD_REQUEST,
            format!("Failed to parse request body: {}", err),
        )
    })?;

    let id = Uuid::new_v4();
    let now: NaiveDateTime = Utc::now().naive_utc();

    let stmt = conn
        .prepare_cached(
            "
            INSERT INTO axum_device (uuid, mac, firmware, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id;",
        )
        .await
        .map_err(internal_error)?;

    let row = conn
        .query_one(&stmt, &[&id, &device.mac, &device.firmware, &now, &now])
        .await
        .map_err(internal_error)?;

    let res = Device {
        id: row.try_get(0).map_err(internal_error)?,
        data: device,
    };

    Ok(Json(res).into_response())
}

fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}
