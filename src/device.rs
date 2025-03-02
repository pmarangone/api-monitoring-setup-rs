use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize)]
pub struct DeviceSchema {
    pub id: Uuid,
    pub mac: String,
    pub firmware: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct Device<'a> {
    pub id: i32,
    #[serde(flatten)]
    pub data: DeviceData<'a>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeviceData<'a> {
    pub mac: &'a str,
    pub firmware: &'a str,
}
