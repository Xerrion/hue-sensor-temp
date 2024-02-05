use chrono::NaiveDateTime;
use diesel::result::Error;
use serde::de::StdError;

use crate::crud::{create_sensor, create_temperature};
use crate::database;
use crate::structs::{HueBridge, HueSensor, Sensors};

impl HueBridge {
    pub fn new(url: String, username: String) -> HueBridge {
        println!(
            "Sensor created with url: {} and username: {}",
            url, username
        );
        HueBridge {
            url,
            username,
            sensors: None,
        }
    }

    pub async fn get_sensors(&mut self) -> Result<Sensors, Box<dyn StdError + Send + Sync>> {
        let url = format!("{}/api/{}/sensors", self.url, self.username);
        let response = reqwest::get(&url).await?;

        let sensor_collection: Sensors = serde_json::from_str(&response.text().await?)?;
        self.sensors = Some(sensor_collection.clone());
        Ok(sensor_collection)
    }

    #[allow(dead_code)]
    pub async fn get_sensor(
        &self,
        sensor_id: i32,
    ) -> Result<HueSensor, Box<dyn std::error::Error>> {
        let sensor_id = sensor_id.to_string();
        let url = format!("{}/api/{}/sensors/{}", self.url, self.username, sensor_id);
        let response = reqwest::get(&url).await?;

        let sensor: HueSensor = serde_json::from_str(&response.text().await?)?;
        Ok(sensor)
    }
}

impl HueSensor {
    pub fn get_temperature(&self) -> Option<f32> {
        match self.state.temperature {
            Some(temperature) => Some(temperature / 100.0),
            None => None,
        }
    }

    pub fn get_lastupdated(&self) -> NaiveDateTime {
        NaiveDateTime::parse_from_str(&self.state.lastupdated, "%Y-%m-%dT%H:%M:%S").unwrap()
    }

    pub fn create_sensor(&self) -> Result<(), Error> {
        match create_sensor(
            database::establish_connection(),
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
        if let Some(temperature) = self.get_temperature() {
            match create_temperature(
                database::establish_connection(),
                temperature,
                sensor_id,
                self.get_lastupdated(),
            ) {
                Ok(_) => Ok(()),
                Err(_) => Ok(()),
            }
        } else {
            Ok(())
        }
    }
}
