use chrono::NaiveDateTime;
use diesel::result::Error;
use std::collections::HashMap;

use crate::crud::{create_sensor, create_temperature};
use crate::database;
use serde::de::StdError;

use crate::structs::{Sensor, SensorDetails, Sensors};

impl SensorDetails {
    pub fn new(url: String, username: String) -> SensorDetails {
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

impl Sensor {
    pub fn get_temperature(&self) -> Option<f32> {
        self.state.temperature
    }

    pub fn get_lastupdated(&self) -> NaiveDateTime {
        NaiveDateTime::parse_from_str(&self.state.lastupdated, "%Y-%m-%dT%H:%M:%S").unwrap()
    }

    pub fn create_sensor(&self) -> Result<(), Error> {
        let mut conn = database::establish_connection();
        match create_sensor(
            &mut conn,
            self.name.clone(),
            self.sensor_type.clone(),
            self.modelid.clone(),
            self.manufacturername.clone(),
            self.swversion.clone(),
            self.uniqueid.clone(),
        ) {
            Ok(_) => Ok(()),
            Err(_) => Ok(()),
        }
    }

    pub fn create_temperature(&self, sensor_id: i32) -> Result<(), Box<dyn std::error::Error>> {
        // Only proceed if there is a temperature value available
        if let Some(temperature) = self.get_temperature() {
            let mut conn = database::establish_connection();

            // Attempt to create the temperature record
            match create_temperature(&mut conn, temperature, sensor_id, self.get_lastupdated()) {
                Ok(_) => Ok(()),
                Err(_) => Ok(()),
            }
        } else {
            Ok(())
        }
    }
}
