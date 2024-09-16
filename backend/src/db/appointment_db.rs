use super::{
    db::Database,
    types::{
        AppointmentRecord, AppointmentRecordWithPatient, AppointmentWithTime, DatabaseError,
        PatientRecordId,
    },
};

impl Database {
    pub async fn create_appointment(
        &self,
        appointment: AppointmentWithTime,
    ) -> Result<Vec<AppointmentRecord>, DatabaseError> {
        let conn = self
            .get_connection()
            .await
            .ok_or(DatabaseError::ConnectionLost)?;

        let patient_exists = self
            .read_patient(appointment.patient_id.get_unique_id())
            .await;
        match patient_exists {
            Ok(_) => {}
            Err(_) => {
                return Err(DatabaseError::NothingFound);
            }
        }

        conn.create("appointment")
            .content(appointment)
            .await
            .map_err(DatabaseError::from)
    }

    async fn populate_appointments_with_patient(
        &self,
        appointments: Vec<AppointmentRecord>,
    ) -> Result<Vec<AppointmentRecordWithPatient>, DatabaseError> {
        let mut appointments_with_patient = Vec::new();

        for appointment in appointments {
            let patient = self
                .read_patient(appointment.patient_id.get_unique_id())
                .await?;
            let appointment_with_patient =
                AppointmentRecordWithPatient::from_appointment_record(appointment, patient);
            appointments_with_patient.push(appointment_with_patient);
        }

        Ok(appointments_with_patient)
    }

    pub async fn read_all_appointments(
        &self,
    ) -> Result<Vec<AppointmentRecordWithPatient>, DatabaseError> {
        let conn = self
            .get_connection()
            .await
            .ok_or(DatabaseError::ConnectionLost)?;

        let appointments: Vec<AppointmentRecord> = conn
            .select("appointment")
            .await
            .map_err(DatabaseError::from)?;

        let appointments_with_patient = self
            .populate_appointments_with_patient(appointments)
            .await?;

        Ok(appointments_with_patient)
    }

    pub async fn read_all_appointments_by_day(
        &self,
        day: &str,
    ) -> Result<Vec<AppointmentRecordWithPatient>, DatabaseError> {
        let conn = self
            .get_connection()
            .await
            .ok_or(DatabaseError::ConnectionLost)?;

        let mut result = conn
            .query("SELECT * FROM appointment WHERE string::startsWith(start_time, $day)")
            .bind(("day", day))
            .await
            .map_err(DatabaseError::from)?;

        let appointments: Vec<AppointmentRecord> = result.take(0)?;

        let appointments_with_patient = self
            .populate_appointments_with_patient(appointments)
            .await?;

        Ok(appointments_with_patient)
    }

    pub async fn read_all_appointments_by_month(
        &self,
        month: &str,
    ) -> Result<Vec<AppointmentRecordWithPatient>, DatabaseError> {
        let conn = self
            .get_connection()
            .await
            .ok_or(DatabaseError::ConnectionLost)?;

        let mut result = conn
            .query("SELECT * FROM appointment WHERE string::startsWith(start_time, $month)")
            .bind(("month", month))
            .await
            .map_err(DatabaseError::from)?;

        let appointments: Vec<AppointmentRecord> = result.take(0)?;

        let appointments_with_patient = self
            .populate_appointments_with_patient(appointments)
            .await?;

        Ok(appointments_with_patient)
    }

    pub async fn read_all_appointments_by_doctor(
        &self,
        doctor_id: &u32,
    ) -> Result<Vec<AppointmentRecordWithPatient>, DatabaseError> {
        let conn = self
            .get_connection()
            .await
            .ok_or(DatabaseError::ConnectionLost)?;

        let mut result = conn
            .query("SELECT * FROM appointment WHERE doctor = $doctor_id")
            .bind(("doctor_id", doctor_id))
            .await
            .map_err(DatabaseError::from)?;

        let appointments: Vec<AppointmentRecord> = result.take(0)?;

        let appointments_with_patient = self
            .populate_appointments_with_patient(appointments)
            .await?;

        Ok(appointments_with_patient)
    }

    pub async fn read_all_appointments_by_room(
        &self,
        room_nr: &u32,
    ) -> Result<Vec<AppointmentRecordWithPatient>, DatabaseError> {
        let conn = self
            .get_connection()
            .await
            .ok_or(DatabaseError::ConnectionLost)?;

        let mut result = conn
            .query("SELECT * FROM appointment WHERE room_nr = $room_nr")
            .bind(("room_nr", room_nr))
            .await
            .map_err(DatabaseError::from)?;

        let appointments: Vec<AppointmentRecord> = result.take(0)?;

        let appointments_with_patient = self
            .populate_appointments_with_patient(appointments)
            .await?;

        Ok(appointments_with_patient)
    }

    pub async fn read_all_appointments_by_patient(
        &self,
        patient_id: &PatientRecordId,
    ) -> Result<Vec<AppointmentRecordWithPatient>, DatabaseError> {
        let conn = self
            .get_connection()
            .await
            .ok_or(DatabaseError::ConnectionLost)?;

        let mut result = conn
            .query("SELECT * FROM appointment WHERE patient_id = $patient_id")
            .bind(("patient_id", patient_id.as_str()))
            .await
            .map_err(DatabaseError::from)?;

        let appointments: Vec<AppointmentRecord> = result.take(0)?;

        let appointments_with_patient = self
            .populate_appointments_with_patient(appointments)
            .await?;

        Ok(appointments_with_patient)
    }

    pub async fn read_appointment(
        &self,
        id: &str,
    ) -> Result<Option<AppointmentRecordWithPatient>, DatabaseError> {
        let conn = self
            .get_connection()
            .await
            .ok_or(DatabaseError::ConnectionLost)?;

        let appointment: AppointmentRecord = conn
            .select(("appointment", id))
            .await
            .map_err(DatabaseError::from)?
            .ok_or(DatabaseError::NothingFound)?;

        let patient = self
            .read_patient(appointment.patient_id.get_unique_id())
            .await?;

        Ok(Some(AppointmentRecordWithPatient::from_appointment_record(
            appointment,
            patient,
        )))
    }

    pub async fn update_appointment(
        &self,
        id: &str,
        appointment: AppointmentRecord,
    ) -> Result<AppointmentRecord, DatabaseError> {
        let conn = self
            .get_connection()
            .await
            .ok_or(DatabaseError::ConnectionLost)?;

        let result = conn
            .update(("appointment", id))
            .merge(appointment)
            .await
            .map_err(DatabaseError::from)?;

        result.ok_or(DatabaseError::NothingFound)
    }

    pub async fn delete_appointment(&self, id: &str) -> Result<AppointmentRecord, DatabaseError> {
        let conn = self
            .get_connection()
            .await
            .ok_or(DatabaseError::ConnectionLost)?;

        let result: Option<AppointmentRecord> = conn
            .delete(("appointment", id))
            .await
            .map_err(DatabaseError::from)?;

        result.ok_or(DatabaseError::NothingFound)
    }
}

#[cfg(test)]
mod appointment_db_tests {
    use crate::db::{
        db::database_tests::mock_db,
        types::{Appointment, AppointmentType, Patient},
    };

    use chrono::NaiveDateTime;
    use rand::{thread_rng, Rng};

    use super::*;

    async fn create_dummy_patients(db: &Database, count: u32) -> Vec<PatientRecordId> {
        let mut patient_ids = Vec::new();
        for i in 0..count {
            let patient = Patient {
                name: format!("John Doe {}", i),
                phone_number: thread_rng().gen_range(100000000..999999999).to_string(),
                insurance_number: None,
            };

            let patient = db.create_patient(patient).await.unwrap();
            patient_ids.push(PatientRecordId::new(&patient[0].id.id.to_raw()));
        }
        patient_ids
    }

    #[tokio::test]
    async fn test_create_appointment() {
        let mock_db = mock_db().await;

        let patient_ids = create_dummy_patients(&mock_db, 2).await;

        let appointment = Appointment {
            start_time: "2023-10-01T10:00:00".to_string(),
            appointment_type: AppointmentType::QuickCheckup,
            patient_id: patient_ids[0].clone(),
            doctor: 1,
            room_nr: 0,
        };

        let result = &mock_db
            .create_appointment(appointment.clone().into_appointment_with_time().unwrap())
            .await
            .unwrap()[0];

        assert_eq!(result.patient_id, appointment.patient_id);
        assert_eq!(result.doctor, appointment.doctor);
        assert_eq!(
            result.start_time,
            NaiveDateTime::parse_from_str(&appointment.start_time, "%Y-%m-%dT%H:%M:%S").unwrap()
        );
        assert!(result.end_time == result.start_time + result.appointment_type.duration());
    }

    #[tokio::test]
    async fn test_populate_appointments_with_patient() {
        let mock_db = mock_db().await;

        let patient_ids = create_dummy_patients(&mock_db, 1).await;

        let appointment = Appointment {
            start_time: "2023-10-01T10:00:00".to_string(),
            appointment_type: AppointmentType::QuickCheckup,
            patient_id: patient_ids[0].clone(),
            doctor: 1,
            room_nr: 0,
        };

        let created_appointment = &mock_db
            .create_appointment(appointment.clone().into_appointment_with_time().unwrap())
            .await
            .unwrap()[0];

        let appointments = vec![(*created_appointment).clone()];

        // Populate appointments with patient data
        let result = mock_db
            .populate_appointments_with_patient(appointments)
            .await
            .unwrap();

        // Assert that the populated data matches the expected data
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].id, created_appointment.id);
        assert_eq!(
            result[0].patient.id.id.to_raw(),
            patient_ids[0].get_unique_id()
        );
    }

    #[tokio::test]
    async fn test_read_all_appointments() {
        let mock_db = mock_db().await;

        let patient_ids = create_dummy_patients(&mock_db, 2).await;

        let appointment1 = Appointment {
            start_time: "2023-10-01T10:00:00".to_string(),
            appointment_type: AppointmentType::QuickCheckup,
            patient_id: patient_ids[0].clone(),
            doctor: 1,
            room_nr: 0,
        };

        let appointment2 = Appointment {
            start_time: "2023-10-01T13:00:00".to_string(),
            appointment_type: AppointmentType::Surgery,
            patient_id: patient_ids[1].clone(),
            doctor: 0,
            room_nr: 1,
        };

        mock_db
            .create_appointment(appointment1.clone().into_appointment_with_time().unwrap())
            .await
            .unwrap();
        mock_db
            .create_appointment(appointment2.clone().into_appointment_with_time().unwrap())
            .await
            .unwrap();

        // Read all appointments from the database
        let result = mock_db.read_all_appointments().await.unwrap();

        // Assert that the retrieved data matches the inserted data
        assert_eq!(result.len(), 2);

        let mut sorted_result = result.clone();
        sorted_result.sort_by_key(|a| a.start_time);

        let mut sorted_appointments = vec![appointment1.clone(), appointment2.clone()];
        sorted_appointments.sort_by_key(|a| {
            NaiveDateTime::parse_from_str(&a.start_time, "%Y-%m-%dT%H:%M:%S").unwrap()
        });

        assert_eq!(
            sorted_result[0].patient.id.id.to_raw(),
            sorted_appointments[0].patient_id.get_unique_id()
        );
        assert_eq!(sorted_result[0].doctor, sorted_appointments[0].doctor);
        assert_eq!(
            sorted_result[0].start_time,
            NaiveDateTime::parse_from_str(&sorted_appointments[0].start_time, "%Y-%m-%dT%H:%M:%S")
                .unwrap()
        );
        assert_eq!(
            sorted_result[1].patient.id.id.to_raw(),
            sorted_appointments[1].patient_id.get_unique_id()
        );
        assert_eq!(sorted_result[1].doctor, sorted_appointments[1].doctor);
        assert_eq!(
            sorted_result[1].start_time,
            NaiveDateTime::parse_from_str(&sorted_appointments[1].start_time, "%Y-%m-%dT%H:%M:%S")
                .unwrap()
        );
    }

    #[tokio::test]
    async fn test_read_all_appointments_filters() {
        let mock_db = mock_db().await;

        let patient_ids = create_dummy_patients(&mock_db, 1).await;

        let appointment1 = Appointment {
            start_time: "2023-10-01T10:00:00".to_string(),
            appointment_type: AppointmentType::QuickCheckup,
            patient_id: patient_ids[0].clone(),
            doctor: 1,
            room_nr: 0,
        };

        let appointment2 = Appointment {
            start_time: "2023-10-15T13:00:00".to_string(),
            appointment_type: AppointmentType::Surgery,
            patient_id: patient_ids[0].clone(),
            doctor: 2,
            room_nr: 1,
        };

        mock_db
            .create_appointment(appointment1.clone().into_appointment_with_time().unwrap())
            .await
            .unwrap();
        mock_db
            .create_appointment(appointment2.clone().into_appointment_with_time().unwrap())
            .await
            .unwrap();

        // Test read_all_appointments_by_day
        let result_by_day = mock_db
            .read_all_appointments_by_day("2023-10-01")
            .await
            .unwrap();
        assert_eq!(result_by_day.len(), 1);

        // Test read_all_appointments_by_month
        let result_by_month = mock_db
            .read_all_appointments_by_month("2023-10")
            .await
            .unwrap();
        assert_eq!(result_by_month.len(), 2);

        // Test read_all_appointments_by_doctor
        let result_by_doctor = mock_db.read_all_appointments_by_doctor(&1).await.unwrap();
        assert_eq!(result_by_doctor.len(), 1);

        // Test read_all_appointments_by_room
        let result_by_room = mock_db.read_all_appointments_by_room(&0).await.unwrap();
        assert_eq!(result_by_room.len(), 1);

        // Test read_all_appointments_by_patient
        let result_by_patient = mock_db
            .read_all_appointments_by_patient(&patient_ids[0])
            .await
            .unwrap();
        assert_eq!(result_by_patient.len(), 2);
    }

    #[tokio::test]
    async fn test_read_appointment() {
        let mock_db = mock_db().await;

        let patient_ids = create_dummy_patients(&mock_db, 1).await;

        let appointment = Appointment {
            start_time: "2023-10-01T10:00:00".to_string(),
            appointment_type: AppointmentType::QuickCheckup,
            patient_id: patient_ids[0].clone(),
            doctor: 1,
            room_nr: 0,
        };

        let created_appointment = &mock_db
            .create_appointment(appointment.clone().into_appointment_with_time().unwrap())
            .await
            .unwrap()[0];

        // Read the appointment from the database
        let result = mock_db
            .read_appointment(&created_appointment.id.id.to_raw())
            .await
            .unwrap()
            .unwrap();

        // Assert that the retrieved data matches the inserted data
        assert_eq!(
            result.patient.id.id.to_raw(),
            appointment.patient_id.get_unique_id()
        );
        assert_eq!(result.doctor, appointment.doctor);
        assert_eq!(
            result.start_time,
            NaiveDateTime::parse_from_str(&appointment.start_time, "%Y-%m-%dT%H:%M:%S").unwrap()
        );
    }

    #[tokio::test]
    async fn test_update_appointment() {
        let mock_db = mock_db().await;

        let patient_ids = create_dummy_patients(&mock_db, 1).await;

        let appointment = Appointment {
            start_time: "2023-10-01T10:00:00".to_string(),
            appointment_type: AppointmentType::QuickCheckup,
            patient_id: patient_ids[0].clone(),
            doctor: 1,
            room_nr: 0,
        };

        let created_appointment = &mock_db
            .create_appointment(appointment.clone().into_appointment_with_time().unwrap())
            .await
            .unwrap()[0];

        let updated_appointment = AppointmentRecord {
            id: created_appointment.id.clone(),
            start_time: NaiveDateTime::parse_from_str("2023-10-01T11:00:00", "%Y-%m-%dT%H:%M:%S")
                .unwrap(),
            end_time: NaiveDateTime::parse_from_str("2023-10-01T11:15:00", "%Y-%m-%dT%H:%M:%S")
                .unwrap(),
            appointment_type: AppointmentType::QuickCheckup,
            patient_id: patient_ids[0].clone(),
            doctor: 2,
            room_nr: 1,
        };

        // Update the appointment in the database
        let result = mock_db
            .update_appointment(
                &created_appointment.id.id.to_raw(),
                updated_appointment.clone(),
            )
            .await
            .unwrap();

        // Assert that the updated data matches the expected data
        assert_eq!(result.patient_id, updated_appointment.patient_id);
        assert_eq!(result.doctor, updated_appointment.doctor);
        assert_eq!(result.start_time, updated_appointment.start_time);
    }

    #[tokio::test]
    async fn test_delete_appointment() {
        let mock_db = mock_db().await;

        let patient_ids = create_dummy_patients(&mock_db, 1).await;

        let appointment = Appointment {
            start_time: "2023-10-01T10:00:00".to_string(),
            appointment_type: AppointmentType::QuickCheckup,
            patient_id: patient_ids[0].clone(),
            doctor: 1,
            room_nr: 0,
        };

        let created_appointment = &mock_db
            .create_appointment(appointment.clone().into_appointment_with_time().unwrap())
            .await
            .unwrap()[0];

        // Delete the appointment from the database
        let result = mock_db
            .delete_appointment(&created_appointment.id.id.to_raw())
            .await
            .unwrap();

        // Assert that the deleted data matches the inserted data
        assert_eq!(result.patient_id, appointment.patient_id);
        assert_eq!(result.doctor, appointment.doctor);
        assert_eq!(
            result.start_time,
            NaiveDateTime::parse_from_str(&appointment.start_time, "%Y-%m-%dT%H:%M:%S").unwrap()
        );

        // Assert that the appointment is no longer in the database
        let result = mock_db
            .read_appointment(&created_appointment.id.to_raw())
            .await;
        assert!(result.is_err());
    }
}
