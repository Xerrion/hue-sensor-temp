use std::env;
use std::error::Error;
use std::sync::Arc;

use diesel::sqlite::Sqlite;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use tokio::sync::Mutex;

use poll::Poller;
use structs::HueBridge;

mod crud;
mod database;
mod models;
mod poll;
mod schema;
mod sensor;
mod structs;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

fn run_migrations(
    connection: &mut impl MigrationHarness<Sqlite>,
) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    connection.run_pending_migrations(MIGRATIONS)?;
    Ok(())
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    run_migrations(&mut database::establish_connection()).unwrap();

    let hue_url = env::var("HUE_URL").expect("HUE_URL must be set.");
    let hue_username = env::var("HUE_USERNAME").expect("API_KEY must be set.");

    let hue_bridge = HueBridge::new(hue_url, hue_username);
    let mutex_hue_bridge = Arc::new(Mutex::new(hue_bridge));

    let poller = Poller::new(mutex_hue_bridge.clone(), 30); // Poll every 60 seconds

    let polling_task = tokio::spawn({
        let mut poller = poller.clone();
        async move {
            poller.start_sensor_polling().await;
        }
    });

    let update_sensor_table_task = tokio::spawn({
        let mut poller = poller.clone();
        async move {
            poller.start_sensor_creation().await;
        }
    });

    let update_temperature_table_task = tokio::spawn({
        let mut poller = poller.clone();
        async move {
            poller.start_temperature_polling().await;
        }
    });

    tokio::try_join!(
        polling_task,
        update_sensor_table_task,
        update_temperature_table_task
    )
    .unwrap();

    print!("Done!");
}
