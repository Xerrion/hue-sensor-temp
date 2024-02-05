use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
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
#[derive(Debug, Clone)]
pub(crate) struct SensorDetails {
    pub(crate) url: String,
    pub(crate) username: String,
    pub(crate) sensors: Option<Sensors>,
}
pub type Sensors = HashMap<String, Sensor>;
