use chrono::NaiveTime;
use serde::Deserialize;
use std::convert::TryFrom;

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub port: u16,
    pub doctor_amount: u32,
    pub room_amount: u32,
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
