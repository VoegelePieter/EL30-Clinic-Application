use serde::{Deserialize, Serialize};
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

#[derive(Debug, Serialize, Deserialize)]
pub struct PatientRecord {
    id: Thing,
    pub name: String,
    pub phone_number: String,
    pub insurance_number: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Record {
    #[allow(dead_code)]
    id: Thing,
}
