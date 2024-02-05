use std::env;
use std::ffi::OsStr;
use std::path::PathBuf;

use diesel::{Connection, SqliteConnection};

pub fn establish_connection() -> SqliteConnection {
    dotenv::dotenv().ok();

    let database_url = env::var("DATABASE_URL").unwrap_or("sensors.db".to_string());
    let mut database_base = PathBuf::new();

    dirs::config_dir().map(|mut p| {
        p.push("hue-sensors");
        database_base = p;
    });

    let full_path = PathBuf::from(database_base.clone()).join(database_url);
    let full_os_path = OsStr::new(&full_path);

    // Create dirs
    if !database_base.exists() {
        std::fs::create_dir_all(database_base.clone()).unwrap();
    }

    SqliteConnection::establish(full_os_path.to_str().unwrap())
        .unwrap_or_else(|_| panic!("Error connecting to {}", full_os_path.to_str().unwrap()))
}
