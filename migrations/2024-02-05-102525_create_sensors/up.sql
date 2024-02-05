-- Your SQL goes here
CREATE TABLE IF NOT EXISTS "sensors"
(
    "id"             INTEGER PRIMARY KEY AUTOINCREMENT,
    name             VARCHAR NOT NULL,
    sensor_type      VARCHAR NOT NULL,
    modelid          VARCHAR NOT NULL,
    manufacturername VARCHAR NOT NULL,
    swversion        VARCHAR NOT NULL,
    uniqueid         VARCHAR UNIQUE
);