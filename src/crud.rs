use diesel::dsl::exists;
use diesel::result::Error;
use diesel::QueryDsl;
use diesel::RunQueryDsl;
use diesel::{insert_into, Connection};
use diesel::{select, ExpressionMethods};
use diesel::{BoolExpressionMethods, SqliteConnection};

use crate::schema::sensors::dsl::sensors;
use crate::schema::sensors::{name, uniqueid};
use crate::schema::temperatures::dsl::temperatures;

pub fn create_sensor(
    mut conn: SqliteConnection,
    new_sensor_name: String,
    sensor_type: String,
    modelid: String,
    manufacturername: String,
    swversion: String,
    new_sensor_uniqueid: Option<String>,
) -> Result<(), Error> {
    conn.transaction::<_, Error, _>(|conn_inside_transaction| {
        let sensor_exists = select(exists(
            sensors.filter(
                uniqueid
                    .eq(&new_sensor_uniqueid)
                    .or(name.eq(&new_sensor_name)),
            ),
        ))
        .get_result::<bool>(conn_inside_transaction)?;

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
            insert_into(sensors)
                .values(&new_sensor)
                .execute(conn_inside_transaction)?;
        }

        Ok(())
    })
}

pub fn create_temperature(
    mut conn: SqliteConnection,
    temperature: f32,
    sensor_id: i32,
    updated: chrono::NaiveDateTime,
) -> Result<(), Error> {
    use crate::schema::temperatures;

    conn.transaction::<_, Error, _>(|conn_inside_transaction| {
        let temp_exists = select(exists(
            temperatures.filter(temperatures::updated.eq(updated)),
        ))
        .first::<bool>(conn_inside_transaction)?;

        if !temp_exists {
            let new_temperature = crate::models::Temperature {
                id: None,
                temperature,
                sensor_id,
                updated,
            };
            println!("Creating temperature: {:?}", new_temperature);
            insert_into(temperatures::table)
                .values(&new_temperature)
                .execute(conn_inside_transaction)
                .expect("Error saving new temperature");
        }

        Ok(())
    })
}
