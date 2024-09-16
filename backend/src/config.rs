use chrono::NaiveTime;
use serde::Deserialize;
use std::convert::TryFrom;

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub port: u16,
    pub doctor_amount: u32,
    pub room_amount: u32,
    pub namespace: String,
    pub database: String,
    pub database_url: String,
    pub opening_time: NaiveTime,
    pub closing_time: NaiveTime,
    pub break_time: NaiveTime,
}

impl AppConfig {
    pub fn new() -> Self {
        let settings = config::Config::builder()
            .add_source(config::File::with_name("server"))
            .build()
            .expect("Failed to read configuration file");

        settings
            .try_into()
            .expect("Failed to deserialize configuration")
    }

    pub fn is_valid_doctor(&self, doctor: u32) -> bool {
        doctor < self.doctor_amount
    }

    pub fn is_valid_room(&self, room: u32) -> bool {
        room < self.room_amount
    }
}

impl TryFrom<config::Config> for AppConfig {
    type Error = config::ConfigError;

    fn try_from(config: config::Config) -> Result<Self, Self::Error> {
        config.try_deserialize()
    }
}

#[cfg(test)]
pub mod config_tests {
    use super::*;

    pub fn get_test_config() -> AppConfig {
        AppConfig {
            port: 8080,
            doctor_amount: 5,
            room_amount: 10,
            namespace: "test".to_string(),
            database: "test".to_string(),
            database_url: "127.0.0.1:8000".to_string(),
            opening_time: NaiveTime::from_hms_opt(8, 0, 0).unwrap(),
            closing_time: NaiveTime::from_hms_opt(17, 0, 0).unwrap(),
            break_time: NaiveTime::from_hms_opt(12, 0, 0).unwrap(),
        }
    }
}
