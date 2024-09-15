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
