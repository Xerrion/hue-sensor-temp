use std::collections::HashMap;

use serde::de::StdError;

use crate::structs::{Sensor, Sensors};

#[derive(Debug, Clone)]
pub(crate) struct SensorDetails {
    url: String,
    username: String,
    pub sensors: Option<Sensors>,
}

impl SensorDetails {
    pub(crate) fn new(url: String, username: String) -> SensorDetails {
        println!(
            "Sensor created with url: {} and username: {}",
            url, username
        );
        SensorDetails {
            url,
            username,
            sensors: None,
        }
    }

    pub async fn get_sensors(
        &mut self,
    ) -> Result<HashMap<String, Sensor>, Box<dyn StdError + Send + Sync>> {
        let url = format!("{}/api/{}/sensors", self.url, self.username);
        let response = reqwest::get(&url).await?;

        let sensor_collection: Sensors = serde_json::from_str(&response.text().await?)?;
        self.sensors = Some(sensor_collection.clone());
        Ok(sensor_collection)
    }

    #[allow(dead_code)]
    pub async fn get_sensor(&self, sensor_id: i32) -> Result<Sensor, Box<dyn std::error::Error>> {
        let sensor_id = sensor_id.to_string();
        let url = format!("{}/api/{}/sensors/{}", self.url, self.username, sensor_id);
        let response = reqwest::get(&url).await?;

        let sensor: Sensor = serde_json::from_str(&response.text().await?)?;
        Ok(sensor)
    }
}
