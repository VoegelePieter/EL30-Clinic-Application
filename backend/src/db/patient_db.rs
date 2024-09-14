use super::{
    db::Database,
    types::{DatabaseError, Patient, PatientRecord},
};

impl Database {
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

        // Delete the patient
        let result = conn
            .delete(("patient", id))
            .await
            .map_err(DatabaseError::from)?;

        // Check if the patient was deleted
        let patient_record = result.ok_or(DatabaseError::NothingFound)?;

        // Delete any appointments associated with the patient
        conn.query("DELETE FROM appointment WHERE patient_id = $id")
            .bind(("id", id))
            .await
            .map_err(DatabaseError::from)?;

        Ok(patient_record)
    }
}
