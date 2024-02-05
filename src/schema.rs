// @generated automatically by Diesel CLI.

diesel::table! {
    sensors (id) {
        id -> Nullable<Integer>,
        name -> Text,
        sensor_type -> Text,
        modelid -> Text,
        manufacturername -> Text,
        swversion -> Text,
        uniqueid -> Nullable<Text>,
    }
}

diesel::table! {
    temperatures (id) {
        id -> Nullable<Integer>,
        temperature -> Float,
        sensor_id -> Integer,
        updated -> Timestamp,
    }
}

diesel::joinable!(temperatures -> sensors (sensor_id));

diesel::allow_tables_to_appear_in_same_query!(sensors, temperatures,);
