use crate::{config::AppConfig, db::types::AppointmentRecordWithPatient};
use chrono::{Duration, NaiveDateTime};

pub async fn is_valid_timeframe(
    start_time: NaiveDateTime,
    end_time: NaiveDateTime,
    doctor: u32,
    room_nr: u32,
    appointments: &Vec<AppointmentRecordWithPatient>,
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

#[cfg(test)]
mod tests {
    use chrono::{Duration, NaiveDateTime, NaiveTime};

    struct Config {
        opening_time: NaiveTime,
        closing_time: NaiveTime,
        break_time: NaiveTime,
        break_duration: Duration,
    }

    struct Appointment {
        doctor: u32,
        room_nr: u32,
        start_time: NaiveDateTime,
        end_time: NaiveDateTime,
    }

    fn is_valid_timeframe(
        config: &Config,
        doctor: u32,
        room_nr: u32,
        start_time: NaiveDateTime,
        end_time: NaiveDateTime,
        appointments: &[Appointment],
    ) -> Result<(), String> {
        let break_end_time = config.break_time + config.break_duration;

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

        for appointment in appointments {
            if (appointment.doctor == doctor || appointment.room_nr == room_nr)
                && (start_time < appointment.end_time && end_time > appointment.start_time)
            {
                return Err("Appointment overlaps with another appointment".to_string());
            }
        }

        Ok(())
    }

    #[test]
    fn test_appointment_outside_opening_hours() {
        let config = Config {
            opening_time: NaiveTime::from_hms_opt(9, 0, 0).unwrap(),
            closing_time: NaiveTime::from_hms_opt(17, 0, 0).unwrap(),
            break_time: NaiveTime::from_hms_opt(12, 0, 0).unwrap(),
            break_duration: Duration::minutes(60),
        };

        let start_time =
            NaiveDateTime::parse_from_str("2023-10-01T08:00:00", "%Y-%m-%dT%H:%M:%S").unwrap();
        let end_time =
            NaiveDateTime::parse_from_str("2023-10-01T09:30:00", "%Y-%m-%dT%H:%M:%S").unwrap();
        let appointments = vec![];

        let result = is_valid_timeframe(&config, 1, 101, start_time, end_time, &appointments);
        assert_eq!(
            result,
            Err("Appointment is outside of opening hours".to_string())
        );
    }

    #[test]
    fn test_appointment_during_break_time() {
        let config = Config {
            opening_time: NaiveTime::from_hms_opt(9, 0, 0).unwrap(),
            closing_time: NaiveTime::from_hms_opt(17, 0, 0).unwrap(),
            break_time: NaiveTime::from_hms_opt(12, 0, 0).unwrap(),
            break_duration: Duration::minutes(60),
        };

        let start_time =
            NaiveDateTime::parse_from_str("2023-10-01T12:30:00", "%Y-%m-%dT%H:%M:%S").unwrap();
        let end_time =
            NaiveDateTime::parse_from_str("2023-10-01T13:00:00", "%Y-%m-%dT%H:%M:%S").unwrap();
        let appointments = vec![];

        let result = is_valid_timeframe(&config, 1, 101, start_time, end_time, &appointments);
        assert_eq!(result, Err("Appointment is during break time".to_string()));
    }

    #[test]
    fn test_appointment_spanning_break_time() {
        let config = Config {
            opening_time: NaiveTime::from_hms_opt(9, 0, 0).unwrap(),
            closing_time: NaiveTime::from_hms_opt(17, 0, 0).unwrap(),
            break_time: NaiveTime::from_hms_opt(12, 0, 0).unwrap(),
            break_duration: Duration::minutes(60),
        };

        let start_time =
            NaiveDateTime::parse_from_str("2023-10-01T11:30:00", "%Y-%m-%dT%H:%M:%S").unwrap();
        let end_time =
            NaiveDateTime::parse_from_str("2023-10-01T13:30:00", "%Y-%m-%dT%H:%M:%S").unwrap();
        let appointments = vec![];

        let result = is_valid_timeframe(&config, 1, 101, start_time, end_time, &appointments);
        assert_eq!(
            result,
            Err("Appointment cannot span across break time".to_string())
        );
    }

    #[test]
    fn test_appointment_overlapping_another() {
        let config = Config {
            opening_time: NaiveTime::from_hms_opt(9, 0, 0).unwrap(),
            closing_time: NaiveTime::from_hms_opt(17, 0, 0).unwrap(),
            break_time: NaiveTime::from_hms_opt(12, 0, 0).unwrap(),
            break_duration: Duration::minutes(60),
        };

        let start_time =
            NaiveDateTime::parse_from_str("2023-10-01T10:00:00", "%Y-%m-%dT%H:%M:%S").unwrap();
        let end_time =
            NaiveDateTime::parse_from_str("2023-10-01T11:00:00", "%Y-%m-%dT%H:%M:%S").unwrap();
        let appointments = vec![Appointment {
            doctor: 1,
            room_nr: 101,
            start_time: NaiveDateTime::parse_from_str("2023-10-01T10:30:00", "%Y-%m-%dT%H:%M:%S")
                .unwrap(),
            end_time: NaiveDateTime::parse_from_str("2023-10-01T11:30:00", "%Y-%m-%dT%H:%M:%S")
                .unwrap(),
        }];

        let result = is_valid_timeframe(&config, 1, 101, start_time, end_time, &appointments);
        assert_eq!(
            result,
            Err("Appointment overlaps with another appointment".to_string())
        );
    }

    #[test]
    fn test_valid_appointment() {
        let config = Config {
            opening_time: NaiveTime::from_hms_opt(9, 0, 0).unwrap(),
            closing_time: NaiveTime::from_hms_opt(17, 0, 0).unwrap(),
            break_time: NaiveTime::from_hms_opt(12, 0, 0).unwrap(),
            break_duration: Duration::minutes(60),
        };

        let start_time =
            NaiveDateTime::parse_from_str("2023-10-01T10:00:00", "%Y-%m-%dT%H:%M:%S").unwrap();
        let end_time =
            NaiveDateTime::parse_from_str("2023-10-01T11:00:00", "%Y-%m-%dT%H:%M:%S").unwrap();
        let appointments = vec![];

        let result = is_valid_timeframe(&config, 1, 101, start_time, end_time, &appointments);
        assert_eq!(result, Ok(()));
    }
}
