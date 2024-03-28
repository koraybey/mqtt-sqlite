use serde::Deserialize;

use crate::schema::*;

#[derive(Insertable, Deserialize)]
#[diesel(table_name = logs)]
pub struct Payload {
    pub friendly_name: Option<String>,
    pub current: Option<f64>,
    pub energy: Option<f64>,
    pub power: Option<f64>,
    pub last_seen: Option<String>,
    pub voltage: Option<i32>,
    pub linkquality: Option<i32>,
    pub state: Option<String>,
    pub contact: Option<bool>,
    pub occupancy: Option<bool>,
    pub battery: Option<i32>,
    pub illuminance: Option<i32>,
    pub device_temperature: Option<f64>,
    pub power_outage_count: Option<i32>,
}

#[derive(Deserialize)]
pub struct Device {
    pub topic: String,
    pub alias: String,
    pub device_type: String,
    pub qos: i32,
}

#[derive(Deserialize)]
pub struct Configuration {
    pub host: String,
    pub protocol: String,
    pub port: i32,
    pub devices: Vec<Device>,
}
