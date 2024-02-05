use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::schema::{sensors, temperatures};

#[derive(Serialize, Deserialize, Insertable, Identifiable, Queryable, PartialEq, Debug)]
#[diesel(table_name = sensors)]
pub struct Sensor {
    pub id: Option<i32>,
    pub name: String,
    pub sensor_type: String,
    pub modelid: String,
    pub manufacturername: String,
    pub swversion: String,
    pub uniqueid: Option<String>,
}

#[derive(Queryable, Insertable, Identifiable, Selectable, Associations, Debug, PartialEq)]
#[diesel(belongs_to(Sensor))]
#[diesel(table_name = temperatures)]
pub struct Temperature {
    pub id: Option<i32>,
    pub temperature: f32,
    pub sensor_id: i32,
    pub updated: NaiveDateTime,
}
