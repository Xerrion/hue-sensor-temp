use std::sync::Arc;

use chrono::Local;
use tokio::sync::Mutex;
use tokio::time::{interval, Duration};

// Adjust the path according to your project structure
use crate::structs::SensorDetails;
use futures::stream::{self, StreamExt};

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
    fn seconds_to_add_for_hour(&self) -> u64 {
        let hour_in_seconds: u64 = 3600;
        if self.interval_seconds >= hour_in_seconds {
            0
        } else {
            hour_in_seconds - self.interval_seconds
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
        let mut interval_timer = interval(Duration::from_secs(
            self.interval_seconds + self.seconds_to_add_for_hour(),
        ));

        loop {
            interval_timer.tick().await;
            println!("Creating sensors {}", Local::now());
            self.create_sensor().await.expect("Sensor creation failed");
            let next_poll = Local::now() + interval_timer.period();
            println!("Next sensor creation at {}", next_poll);
        }
    }

    pub async fn start_temperature_polling(&mut self) {
        let mut interval_timer = interval(Duration::from_secs(self.interval_seconds));

        loop {
            interval_timer.tick().await;

            println!("Polling temperature {}", Local::now());
            self.create_temperature()
                .await
                .expect("Temperature creation failed");
            let next_poll = Local::now() + interval_timer.period();
            println!("Next temperature poll at {}", next_poll);
        }
    }

    async fn poll_sensors(&mut self) {
        let mut sensor_details_guard = self.sensors.lock().await;
        if let Err(e) = sensor_details_guard.get_sensors().await {
            eprintln!("Error polling sensors: {}", e);
            return;
        }
    }

    async fn create_sensor(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut sensor_details_guard = self.sensors.lock().await;
        let sensors = sensor_details_guard.get_sensors().await;

        stream::iter(sensors.unwrap())
            .for_each_concurrent(None, |(_, sensor)| async move {
                if let Err(e) = sensor.create_sensor() {
                    eprintln!("Error creating sensor: {}", e);
                }
            })
            .await;
        Ok(())
    }

    async fn create_temperature(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let mut sensor_details_guard = self.sensors.lock().await;
        let sensors = sensor_details_guard.get_sensors().await;

        stream::iter(sensors.unwrap())
            .for_each_concurrent(None, |(id, sensor)| async move {
                if let Err(e) = sensor.create_temperature(id.parse().unwrap()) {
                    eprintln!("Error creating temperature: {}", e);
                }
            })
            .await;
        Ok(())
    }
}
