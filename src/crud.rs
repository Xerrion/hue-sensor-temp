use diesel::SqliteConnection;

pub fn create_sensor(
    conn: &mut SqliteConnection,
    name: String,
    sensor_type: String,
    modelid: String,
    manufacturername: String,
    swversion: String,
    uniqueid: Option<String>,
) {
    use crate::schema::sensors;
    use diesel::dsl::exists;
    use diesel::expression_methods::ExpressionMethods;
    use diesel::query_dsl::QueryDsl;
    use diesel::RunQueryDsl;

    let new_sensor = crate::models::Sensor {
        id: None,
        name: name.clone(),
        sensor_type,
        modelid,
        manufacturername,
        swversion,
        uniqueid: uniqueid.clone(),
    };

    let select_by_uniqueid = sensors::columns::uniqueid.eq(&uniqueid);
    let select_by_name = sensors::columns::name.eq(&name);
    let query = sensors::table
        .filter(select_by_uniqueid)
        .or_filter(select_by_name);

    match diesel::select(exists(query)).get_result(conn) {
        Ok(true) => {}
        Ok(false) => {
            println!("Creating sensor: {:?}", new_sensor);
            // Insert the new sensor into the database, if the uniqueid doesn't exist
            diesel::insert_into(sensors::table)
                .values(new_sensor)
                .execute(conn)
                .expect("Error saving new sensor");
        }
        Err(e) => {
            println!("Error checking for sensor: {}", e);
        }
    }
}

pub fn create_temperature(
    conn: &mut SqliteConnection,
    temperature: f32,
    sensor_id: i32,
    updated: chrono::NaiveDateTime,
) {
    use crate::schema::temperatures;
    use diesel::RunQueryDsl;

    let new_temperature = crate::models::Temperature {
        id: None,
        temperature,
        sensor_id,
        updated,
    };

    println!("Creating temperature: {:?}", new_temperature);
    diesel::insert_into(temperatures::table)
        .values(new_temperature)
        .execute(conn)
        .expect("Error saving new sensor");
}
