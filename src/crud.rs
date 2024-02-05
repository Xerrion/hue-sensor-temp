use diesel::dsl::exists;
use diesel::result::Error;
use diesel::Connection;
use diesel::ExpressionMethods;
use diesel::QueryDsl;
use diesel::RunQueryDsl;
use diesel::{BoolExpressionMethods, SqliteConnection};

use crate::schema::sensors::dsl::sensors;
use crate::schema::sensors::{name, uniqueid};

pub fn create_sensor(
    conn: &mut SqliteConnection,
    new_sensor_name: String,
    sensor_type: String,
    modelid: String,
    manufacturername: String,
    swversion: String,
    new_sensor_uniqueid: Option<String>,
) -> Result<(), Error> {
    // Adjust the closure to use the connection provided by the transaction method
    conn.transaction::<_, Error, _>(|conn_inside_transaction| {
        let sensor_exists = diesel::select(exists(
            sensors.filter(
                uniqueid
                    .eq(&new_sensor_uniqueid)
                    .or(name.eq(&new_sensor_name)),
            ),
        ))
        .get_result::<bool>(conn_inside_transaction)?; // Use the connection provided to the closure

        if !sensor_exists {
            let new_sensor = crate::models::Sensor {
                id: None,
                name: new_sensor_name,
                sensor_type,
                modelid,
                manufacturername,
                swversion,
                uniqueid: new_sensor_uniqueid,
            };
            println!("Creating sensor: {:?}", new_sensor);
            diesel::insert_into(sensors)
                .values(&new_sensor)
                .execute(conn_inside_transaction)?; // Use the connection provided to the closure
        }

        Ok(())
    })
}

pub fn create_temperature(
    conn: &mut SqliteConnection,
    temperature: f32,
    sensor_id: i32,
    updated: chrono::NaiveDateTime,
) -> Result<(), Error> {
    use crate::schema::temperatures;

    conn.transaction::<_, Error, _>(|conn_inside_transaction| {
        let new_temperature = crate::models::Temperature {
            id: None,
            temperature,
            sensor_id,
            updated,
        };

        println!("Creating temperature: {:?}", new_temperature);
        diesel::insert_into(temperatures::table)
            .values(&new_temperature)
            .execute(conn_inside_transaction)
            .expect("Error saving new temperature");
        Ok(())
    })
}
