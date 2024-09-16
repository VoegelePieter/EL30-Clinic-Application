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

#[cfg(test)]
mod patient_db_tests {
    use crate::db::db::database_tests::mock_db;

    use super::*;

    #[tokio::test]
    async fn test_create_patient_success() {
        let mock_db = mock_db().await;

        let patient = Patient {
            name: "John Doe".to_string(),
            phone_number: "1234567890".to_string(),
            insurance_number: None,
        };

        let result = mock_db.create_patient(patient.clone()).await.unwrap();

        assert_eq!(result.len(), 1);
        assert!(!result[0].id.to_string().is_empty());
        assert_eq!(result[0].name, patient.name);
        assert_eq!(result[0].phone_number, patient.phone_number);
        assert_eq!(result[0].insurance_number, patient.insurance_number);
    }

    #[tokio::test]
    async fn test_read_all_patients_success() {
        let mock_db = mock_db().await;

        let patient1 = Patient {
            name: "John Doe".to_string(),
            phone_number: "1234567890".to_string(),
            insurance_number: None,
        };

        let patient2 = Patient {
            name: "Jane Smith".to_string(),
            phone_number: "0987654321".to_string(),
            insurance_number: Some("INS123456".to_string()),
        };

        mock_db.create_patient(patient1.clone()).await.unwrap();
        mock_db.create_patient(patient2.clone()).await.unwrap();

        let result = mock_db.read_all_patients().await.unwrap();

        assert_eq!(result.len(), 2);
        assert!(result.iter().any(|p| p.name == patient1.name
            && p.phone_number == patient1.phone_number
            && p.insurance_number == patient1.insurance_number));
        assert!(result.iter().any(|p| p.name == patient2.name
            && p.phone_number == patient2.phone_number
            && p.insurance_number == patient2.insurance_number));
    }

    #[tokio::test]
    async fn test_read_patient() {
        let mock_db = mock_db().await;

        let patient1 = Patient {
            name: "John Doe".to_string(),
            phone_number: "1234567890".to_string(),
            insurance_number: None,
        };

        let patient2 = Patient {
            name: "Jane Smith".to_string(),
            phone_number: "0987654321".to_string(),
            insurance_number: Some("INS123456".to_string()),
        };

        let to_read = mock_db.create_patient(patient1.clone()).await.unwrap();
        mock_db.create_patient(patient2.clone()).await.unwrap();

        // Read the patient data from the database
        let result = mock_db
            .read_patient(&to_read[0].id.id.to_raw())
            .await
            .unwrap();

        // Assert that the retrieved data matches the inserted data
        assert_eq!(result.name, patient1.name);
        assert_eq!(result.phone_number, patient1.phone_number);
        assert_eq!(result.insurance_number, patient1.insurance_number);
    }

    #[tokio::test]
    async fn test_update_patient() {
        let mock_db = mock_db().await;

        let patient = Patient {
            name: "John Doe".to_string(),
            phone_number: "1234567890".to_string(),
            insurance_number: None,
        };

        let to_update = mock_db.create_patient(patient.clone()).await.unwrap();

        let updated_patient = PatientRecord {
            id: to_update[0].id.clone(),
            name: "John Doe Updated".to_string(),
            phone_number: "0987654321".to_string(),
            insurance_number: Some("INS654321".to_string()),
        };

        // Update the patient data in the database
        let result = mock_db
            .update_patient(&to_update[0].id.id.to_raw(), updated_patient.clone())
            .await
            .unwrap();

        // Assert that the updated data matches the expected data
        assert_eq!(result.name, updated_patient.name);
        assert_eq!(result.phone_number, updated_patient.phone_number);
        assert_eq!(result.insurance_number, updated_patient.insurance_number);
    }

    #[tokio::test]
    async fn test_delete_patient() {
        let mock_db = mock_db().await;

        let patient = Patient {
            name: "John Doe".to_string(),
            phone_number: "1234567890".to_string(),
            insurance_number: None,
        };

        let to_delete = mock_db.create_patient(patient.clone()).await.unwrap();

        // Delete the patient data from the database
        let result = mock_db
            .delete_patient(&to_delete[0].id.id.to_raw())
            .await
            .unwrap();

        // Assert that the deleted data matches the inserted data
        assert_eq!(result.name, patient.name);
        assert_eq!(result.phone_number, patient.phone_number);
        assert_eq!(result.insurance_number, patient.insurance_number);

        // Assert that the patient data is no longer in the database
        let result = mock_db.read_patient(&to_delete[0].id.id.to_raw()).await;

        assert!(result.is_err());
    }
}
