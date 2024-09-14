use crate::{config::AppConfig, db::types::AppointmentRecordWithPatient};
use chrono::{Duration, NaiveDateTime};

pub async fn is_valid_timeframe(
    start_time: NaiveDateTime,
    end_time: NaiveDateTime,
    doctor: u32,
    room_nr: u32,
    appointments: &[AppointmentRecordWithPatient],
    config: &AppConfig,
) -> Result<(), String> {
    // Check if the appointment is within opening hours
    let break_end_time = config.break_time + Duration::hours(1);

    if start_time.time() < config.opening_time || end_time.time() > config.closing_time {
        return Err("Appointment is outside of opening hours".to_string());
    }

    if (start_time.time() >= config.break_time && start_time.time() < break_end_time)
        || (end_time.time() > config.break_time && end_time.time() <= break_end_time)
    {
        return Err("Appointment is during break time".to_string());
    }

    if start_time.time() < config.break_time && end_time.time() > config.break_time {
        return Err("Appointment cannot span across break time".to_string());
    }

    // Check for overlapping appointments
    for appointment in appointments {
        if (appointment.doctor == doctor || appointment.room_nr == room_nr)
            && (start_time < appointment.end_time && end_time > appointment.start_time)
        {
            return Err("Appointment overlaps with another appointment".to_string());
        }
    }

    Ok(())
}
