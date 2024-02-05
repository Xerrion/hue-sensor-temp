-- Your SQL goes here
CREATE TABLE IF NOT EXISTS "temperatures"
(
    "id"          INTEGER PRIMARY KEY AUTOINCREMENT,
    "temperature" REAL    NOT NULL,
    "sensor_id"   INTEGER NOT NULL REFERENCES "sensors" ("id") ON DELETE CASCADE,
    "updated"    TIMESTAMP    NOT NULL
);