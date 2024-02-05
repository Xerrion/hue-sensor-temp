use std::collections::HashMap;

use chrono::NaiveDateTime;
use diesel::result::Error;
use serde::{Deserialize, Serialize};

use crate::crud::{create_sensor, create_temperature};
use crate::database;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Sensor {
    pub state: State,
    pub config: Config,
    pub name: String,
    #[serde(rename = "type")]
    pub sensor_type: String,
    pub modelid: String,
    pub manufacturername: String,
    pub swversion: String,
    pub uniqueid: Option<String>,
    pub recycle: Option<bool>,
    #[serde(flatten)]
    pub swupdate: Option<SoftwareUpdate>,
    #[serde(flatten)]
    pub productname: Option<String>,
    #[serde(flatten)]
    pub diversityid: Option<String>,
    #[serde(flatten)]
    pub capabilities: Option<Capabilities>,
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

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct State {
    pub daylight: Option<bool>,
    pub lastupdated: String,
    pub status: Option<i32>,
    pub presence: Option<bool>,
    pub lightlevel: Option<i32>,
    pub dark: Option<bool>,
    pub temperature: Option<f32>,
    pub buttonevent: Option<i32>,
    pub expectedrotation: Option<i32>,
    pub expectedeventduration: Option<i32>,
    pub rotaryevent: Option<i32>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config {
    pub on: bool,
    pub battery: Option<i32>,
    pub reachable: Option<bool>,
    pub alert: Option<String>,
    pub sensitivity: Option<i32>,
    pub sensitivitymax: Option<i32>,
    pub ledindication: Option<bool>,
    pub usertest: Option<bool>,
    pub pending: Option<Vec<String>>,
    pub sunriseoffset: Option<i32>,
    pub sunsetoffset: Option<i32>,
    pub configured: Option<bool>,
    pub tholddark: Option<i32>,
    pub tholdoffset: Option<i32>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SoftwareUpdate {
    pub state: String,
    pub lastinstall: String,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Capabilities {
    pub certified: bool,
    pub primary: Option<bool>,
    #[serde(flatten)]
    pub inputs: Option<Vec<Input>>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Input {
    pub repeatintervals: Vec<i32>,
    #[serde(flatten)]
    pub events: Vec<Event>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Event {
    pub buttonevent: Option<i32>,
    pub eventtype: String,
    pub rotaryevent: Option<i32>,
}

pub type Sensors = HashMap<String, Sensor>;
