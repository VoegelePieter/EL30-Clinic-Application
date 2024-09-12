use super::types::{DatabaseError, Patient, PatientRecord};
use std::sync::{Arc, Mutex};
use surrealdb::{engine::remote::ws::Ws, opt::auth::Root, Surreal};

#[derive(Clone)]
pub struct Database {
    connection: Arc<Mutex<Option<Surreal<surrealdb::engine::remote::ws::Client>>>>,
}

impl Database {
    pub fn new() -> Self {
        Self {
            connection: Arc::new(Mutex::new(None)),
        }
    }

    // CREATION OF DATABASE

    pub async fn initiate_db(&self) -> surrealdb::Result<()> {
        // Connect to the server
        let db = Surreal::new::<Ws>("127.0.0.1:8000").await?;

        // Signin as a namespace, database, or root user
        db.signin(Root {
            username: "root",
            password: "root",
        })
        .await?;

        // Select a specific namespace / database
        db.use_ns("test").use_db("test").await?;

        // Store the connection in the struct
        let mut conn = self.connection.lock().unwrap();
        *conn = Some(db);

        Ok(())
    }

    // CONSISTENT FETCHING OF CONNECTION

    pub async fn get_connection(&self) -> Option<Surreal<surrealdb::engine::remote::ws::Client>> {
        let conn = self.connection.lock().unwrap();
        conn.clone()
    }

    // CRUD OPERATIONS PATIENT

    pub async fn create_patient(
        &self,
        patient: Patient,
    ) -> Result<Vec<PatientRecord>, DatabaseError> {
        let conn = self
            .get_connection()
            .await
            .ok_or(DatabaseError::ConnectionLost)?;

        conn.create("patient")
            .content(patient)
            .await
            .map_err(DatabaseError::from)
    }

    pub async fn read_all_patients(&self) -> Result<Vec<PatientRecord>, DatabaseError> {
        let conn = self
            .get_connection()
            .await
            .ok_or(DatabaseError::ConnectionLost)?;

        conn.select("patient").await.map_err(DatabaseError::from)
    }

    pub async fn read_patient(&self, id: &str) -> Result<PatientRecord, DatabaseError> {
        let conn = self
            .get_connection()
            .await
            .ok_or(DatabaseError::ConnectionLost)?;

        let result = conn
            .select(("patient", id))
            .await
            .map_err(DatabaseError::from)?;

        result.ok_or(DatabaseError::NothingFound)
    }

    pub async fn update_patient(
        &self,
        id: &str,
        patient: PatientRecord,
    ) -> Result<PatientRecord, DatabaseError> {
        let conn = self
            .get_connection()
            .await
            .ok_or(DatabaseError::ConnectionLost)?;

        let result = conn
            .update(("patient", id))
            .merge(patient)
            .await
            .map_err(DatabaseError::from)?;

        result.ok_or(DatabaseError::NothingFound)
    }

    pub async fn delete_patient(&self, id: &str) -> Result<PatientRecord, DatabaseError> {
        let conn = self
            .get_connection()
            .await
            .ok_or(DatabaseError::ConnectionLost)?;

        let result = conn
            .delete(("patient", id))
            .await
            .map_err(DatabaseError::from)?;

        result.ok_or(DatabaseError::NothingFound)
    }
}
