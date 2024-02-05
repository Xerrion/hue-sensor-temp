use std::sync::Arc;

use chrono::Local;
use tokio::sync::Mutex;
use tokio::time::{interval, Duration};

// Adjust the path according to your project structure
use crate::sensor::SensorDetails;

#[derive(Debug, Clone)]
pub struct Poller {
    sensors: Arc<Mutex<SensorDetails>>,
    interval_seconds: u64,
}

impl Poller {
    pub fn new(sensors: Arc<Mutex<SensorDetails>>, interval_seconds: u64) -> Self {
        Poller {
            sensors,
            interval_seconds,
        }
    }

    pub async fn start_sensor_polling(&mut self) {
        let mut interval_timer = interval(Duration::from_secs(self.interval_seconds));

        loop {
            interval_timer.tick().await;

            println!("Polling sensors {}", Local::now());
            self.poll_sensors().await;
            let next_poll = Local::now() + interval_timer.period();
            println!("Next sensor poll at {}", next_poll);
        }
    }

    pub async fn start_sensor_creation(&mut self) {
        let mut interval_timer = interval(Duration::from_secs(self.interval_seconds));

        loop {
            interval_timer.tick().await;
            println!("Creating sensors {}", Local::now());
            self.create_sensor().await;
            let next_poll = Local::now() + interval_timer.period();
            println!("Next sensor creation at {}", next_poll);
        }
    }

    pub async fn start_temperature_polling(&mut self) {
        let mut interval_timer = interval(Duration::from_secs(self.interval_seconds + 60));

        loop {
            interval_timer.tick().await;

            println!("Polling temperature {}", Local::now());
            self.create_temperature().await;
            let next_poll = Local::now() + interval_timer.period();
            println!("Next temperature poll at {}", next_poll);
        }
    }

    async fn poll_sensors(&mut self) {
        let mut sensor_details_guard = self.sensors.lock().await;
        sensor_details_guard.get_sensors().await.unwrap();
    }

    async fn create_sensor(&mut self) {
        let mut sensor_details_guard = self.sensors.lock().await;
        let sensors = sensor_details_guard.get_sensors().await.unwrap();

        sensors
            .iter()
            .for_each(|(_, sensor)| sensor.create_sensor());
    }

    async fn create_temperature(&mut self) {
        let mut sensor_details_guard = self.sensors.lock().await;
        let sensors = sensor_details_guard.get_sensors().await.unwrap();

        sensors.iter().for_each(|(id, sensor)| {
            if sensor.state.temperature.is_some() {
                sensor.create_temperature(id.parse::<i32>().unwrap());
            }
        });
    }
}
