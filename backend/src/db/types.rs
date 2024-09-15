use std::fmt;

use chrono::{Duration, NaiveDateTime};
use serde::{
    de::{self, Visitor},
    Deserialize, Deserializer, Serialize,
};
use surrealdb::sql::Thing;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DatabaseError {
    #[error("Database connection lost")]
    ConnectionLost,
    #[error("No entry found")]
    NothingFound,
    #[error("SurrealDB error: {0}")]
    SurrealDBError(#[from] surrealdb::Error),
    #[error("Chrono parse error: {0}")]
    ChronoError(#[from] chrono::ParseError),
    #[error("Other error: {0}")]
    #[allow(dead_code)]
    Other(String),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Patient {
    pub name: String,
    pub phone_number: String,
    pub insurance_number: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PatientRecord {
    id: Thing,
    pub name: String,
    pub phone_number: String,
    pub insurance_number: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Appointment {
    pub start_time: String,
    pub appointment_type: AppointmentType,
    pub patient_id: PatientRecordId,
    pub doctor: u32,
    pub room_nr: u32,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AppointmentWithTime {
    pub start_time: NaiveDateTime,
    pub end_time: NaiveDateTime,
    pub appointment_type: AppointmentType,
    pub patient_id: PatientRecordId,
    pub doctor: u32,
    pub room_nr: u32,
}

impl Appointment {
    pub fn into_appointment_with_time(self) -> Result<AppointmentWithTime, chrono::ParseError> {
        let start_time = NaiveDateTime::parse_from_str(&self.start_time, "%Y-%m-%dT%H:%M:%S")?;
        let end_time = start_time + self.appointment_type.duration();
        Ok(AppointmentWithTime {
            start_time,
            end_time,
            appointment_type: self.appointment_type,
            patient_id: self.patient_id,
            doctor: self.doctor,
            room_nr: self.room_nr,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AppointmentRecord {
    id: Thing,
    pub start_time: NaiveDateTime,
    pub end_time: NaiveDateTime,
    pub appointment_type: AppointmentType,
    pub patient_id: PatientRecordId,
    pub doctor: u32,
    pub room_nr: u32,
}
impl AppointmentRecord {
    pub fn calculate_end_time(&self) -> NaiveDateTime {
        self.start_time + self.appointment_type.duration()
    }
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppointmentRecordWithPatient {
    pub id: Thing,
    pub start_time: NaiveDateTime,
    pub end_time: NaiveDateTime,
    pub appointment_type: AppointmentType,
    pub patient: PatientRecord,
    pub doctor: u32,
    pub room_nr: u32,
}
impl AppointmentRecordWithPatient {
    pub fn from_appointment_record(
        appointment_record: AppointmentRecord,
        patient_record: PatientRecord,
    ) -> Self {
        AppointmentRecordWithPatient {
            id: appointment_record.id,
            start_time: appointment_record.start_time,
            end_time: appointment_record.end_time,
            appointment_type: appointment_record.appointment_type,
            patient: patient_record,
            doctor: appointment_record.doctor,
            room_nr: appointment_record.room_nr,
        }
    }
    pub fn into_appointment_record(self) -> AppointmentRecord {
        AppointmentRecord {
            id: self.id,
            start_time: self.start_time,
            end_time: self.end_time,
            appointment_type: self.appointment_type,
            patient_id: PatientRecordId::new(&self.patient.id.id.to_raw()),
            doctor: self.doctor,
            room_nr: self.room_nr,
        }
    }
}
#[derive(Debug, Deserialize)]
#[serde(tag = "filter", content = "value", rename_all = "snake_case")]
pub enum AppointmentFilter {
    Month(String),
    Day(String),
    PatientId(PatientRecordId),
    Doctor(u32),
    RoomNr(u32),
}

impl AppointmentFilter {
    pub fn from_filter_request(filter: &str, value: &str) -> Result<Self, String> {
        match filter {
            "month" => Ok(AppointmentFilter::Month(value.to_string())),
            "day" => Ok(AppointmentFilter::Day(value.to_string())),
            "patient_id" => Ok(AppointmentFilter::PatientId(PatientRecordId::new(value))),
            "doctor" => {
                let doctor = value.parse::<u32>().map_err(|e| e.to_string())?;
                Ok(AppointmentFilter::Doctor(doctor))
            }
            "room_nr" => {
                let room_nr = value.parse::<u32>().map_err(|e| e.to_string())?;
                Ok(AppointmentFilter::RoomNr(room_nr))
            }
            _ => Err(format!("Unknown filter type: {}", filter)),
        }
    }
}
#[derive(Debug, Serialize)]
pub struct PatientRecordId(String);

impl PatientRecordId {
    #[allow(dead_code)]
    pub fn new(unique_id: &str) -> Self {
        PatientRecordId(format!("patient:{}", unique_id))
    }

    #[allow(dead_code)]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn get_unique_id(&self) -> &str {
        self.0.split(':').nth(1).unwrap()
    }
}
impl<'de> Deserialize<'de> for PatientRecordId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct PatientRecordIdVisitor;

        impl<'de> Visitor<'de> for PatientRecordIdVisitor {
            type Value = PatientRecordId;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a string formatted as 'patient:{unique_id}'")
            }

            fn visit_str<E>(self, value: &str) -> Result<PatientRecordId, E>
            where
                E: de::Error,
            {
                if value.starts_with("patient:") {
                    Ok(PatientRecordId(value.to_string()))
                } else {
                    Err(de::Error::custom(
                        "ID must be formatted as 'patient:{unique_id}'",
                    ))
                }
            }
        }

        deserializer.deserialize_str(PatientRecordIdVisitor)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum AppointmentType {
    QuickCheckup,
    ExtensiveCare,
    Surgery,
}

impl AppointmentType {
    pub fn duration(&self) -> Duration {
        match self {
            AppointmentType::QuickCheckup => Duration::minutes(30),
            AppointmentType::ExtensiveCare => Duration::hours(1),
            AppointmentType::Surgery => Duration::hours(2),
        }
    }
}
