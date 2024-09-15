use std::sync::{Arc, Mutex};

use actix_web::{web, HttpResponse, Responder};
use chrono::NaiveDateTime;
use serde::Deserialize;

use crate::db::types::{AppointmentFilter, AppointmentRecordWithPatient};
use crate::types::ApiResponse;
use crate::util::is_valid_timeframe;
use crate::{
    config::AppConfig,
    db::{
        db::Database,
        types::{Appointment, AppointmentRecord, AppointmentType, DatabaseError},
    },
};

// Appointment Types
#[derive(Deserialize)]
pub struct AppointmentId {
    id: String,
}
#[derive(Deserialize)]
pub struct UpdateAppointment {
    start_time: Option<NaiveDateTime>,
    appointment_type: Option<AppointmentType>,
    doctor: Option<u32>,
    room_nr: Option<u32>,
}
#[derive(Deserialize, Debug)]
pub struct FilterRequest {
    filter: Option<String>,
    value: Option<String>,
}

// Endpoints
pub async fn read_all_appointments_handler(
    database: web::Data<Arc<Mutex<Database>>>,
    filter_request: web::Query<FilterRequest>,
) -> impl Responder {
    if let (Some(filter), Some(value)) = (&filter_request.filter, &filter_request.value) {
        let appointment_filter: AppointmentFilter =
            match AppointmentFilter::from_filter_request(filter, value) {
                Ok(filter) => filter,
                Err(e) => return HttpResponse::BadRequest().body(format!("Error: {:?}", e)),
            };
        read_all_appointments_by_filter(database, &appointment_filter).await
    } else {
        read_all_appointments(database).await
    }
}

pub async fn read_all_appointments(database: web::Data<Arc<Mutex<Database>>>) -> HttpResponse {
    let db = match database.lock() {
        Ok(guard) => guard,
        Err(err) => {
            return HttpResponse::InternalServerError().body(format!("Lock error: {:?}", err))
        }
    };

    match db.read_all_appointments().await {
        Ok(appointments) => HttpResponse::Ok().json(ApiResponse { data: appointments }),
        Err(err) => HttpResponse::InternalServerError().body(format!("Error: {:?}", err)),
    }
}

pub async fn read_all_appointments_by_filter(
    database: web::Data<Arc<Mutex<Database>>>,
    filter_request: &AppointmentFilter,
) -> HttpResponse {
    let db = match database.lock() {
        Ok(guard) => guard,
        Err(err) => {
            return HttpResponse::InternalServerError().body(format!("Lock error: {:?}", err))
        }
    };

    let filtered_appointments: Vec<AppointmentRecordWithPatient> = match &filter_request {
        AppointmentFilter::Day(day) => {
            if let Err(err) =
                NaiveDateTime::parse_from_str(&format!("{} 00:00:00", day), "%Y-%m-%d %H:%M:%S")
            {
                return HttpResponse::BadRequest().body(format!(
                    "Invalid day format: {:?}\nCorrect format is Y-m-d",
                    err
                ));
            }
            match db.read_all_appointments_by_day(day).await {
                Ok(appointments) => appointments,
                Err(err) => {
                    return HttpResponse::InternalServerError().body(format!("Error: {:?}", err))
                }
            }
        }
        AppointmentFilter::Month(month) => {
            if let Err(err) = NaiveDateTime::parse_from_str(
                &format!("{}-01 00:00:00", month),
                "%Y-%m-%d %H:%M:%S",
            ) {
                return HttpResponse::BadRequest().body(format!(
                    "Invalid month format: {:?}\nCorrect format is Y-m",
                    err
                ));
            }
            match db.read_all_appointments_by_month(month).await {
                Ok(appointments) => appointments,
                Err(err) => {
                    return HttpResponse::InternalServerError().body(format!("Error: {:?}", err))
                }
            }
        }
        AppointmentFilter::PatientId(patient) => {
            match db.read_all_appointments_by_patient(patient).await {
                Ok(appointments) => appointments,
                Err(err) => {
                    return HttpResponse::InternalServerError().body(format!("Error: {:?}", err))
                }
            }
        }
        AppointmentFilter::Doctor(doctor) => {
            match db.read_all_appointments_by_doctor(doctor).await {
                Ok(appointments) => appointments,
                Err(err) => {
                    return HttpResponse::InternalServerError().body(format!("Error: {:?}", err))
                }
            }
        }
        AppointmentFilter::RoomNr(room) => match db.read_all_appointments_by_room(room).await {
            Ok(appointments) => appointments,
            Err(err) => {
                return HttpResponse::InternalServerError().body(format!("Error: {:?}", err))
            }
        },
    };

    HttpResponse::Ok().json(ApiResponse {
        data: filtered_appointments,
    })
}

pub async fn create_appointment(
    database: web::Data<Arc<Mutex<Database>>>,
    config: web::Data<AppConfig>,
    appointment: web::Json<Appointment>,
) -> impl Responder {
    let db = match database.lock() {
        Ok(guard) => guard,
        Err(err) => {
            return HttpResponse::InternalServerError().body(format!("Lock error: {:?}", err))
        }
    };

    if !config.is_valid_doctor(appointment.doctor) || !config.is_valid_room(appointment.room_nr) {
        return HttpResponse::BadRequest().body(format!("Doctor or room not found. The configured maximum doctor is {}, and the configured maximum room is {}.\nCount starts at 0", config.doctor_amount - 1, config.room_amount - 1));
    }

    let appointment_with_calculated_time =
        match appointment.into_inner().into_appointment_with_time() {
            Ok(appointment) => appointment,
            Err(e) => {
                return HttpResponse::InternalServerError()
                    .body(format!("Error: {:?}", DatabaseError::from(e)))
            }
        };

    let all_appointments: Vec<AppointmentRecordWithPatient> = match db.read_all_appointments().await
    {
        Ok(appointments) => appointments,
        Err(err) => return HttpResponse::InternalServerError().body(format!("Error: {:?}", err)),
    };

    match is_valid_timeframe(
        appointment_with_calculated_time.start_time,
        appointment_with_calculated_time.end_time,
        appointment_with_calculated_time.doctor,
        appointment_with_calculated_time.room_nr,
        &all_appointments,
        &config,
    )
    .await
    {
        Ok(_) => {
            match db
                .create_appointment(appointment_with_calculated_time)
                .await
            {
                Ok(result) => HttpResponse::Ok().json(ApiResponse { data: result }),
                Err(err) => HttpResponse::InternalServerError().body(format!("Error: {:?}", err)),
            }
        }
        Err(e) => HttpResponse::BadRequest().body(format!("Error: {:?}", e)),
    }
}

pub async fn delete_appointment(
    database: web::Data<Arc<Mutex<Database>>>,
    appointment_id: web::Path<AppointmentId>,
) -> impl Responder {
    let db = match database.lock() {
        Ok(guard) => guard,
        Err(err) => {
            return HttpResponse::InternalServerError().body(format!("Lock error: {:?}", err))
        }
    };
    match db.delete_appointment(&appointment_id.id).await {
        Ok(appointments) => HttpResponse::Ok().json(ApiResponse { data: appointments }),
        Err(err) => match err {
            DatabaseError::NothingFound => HttpResponse::NotFound().body("Appointment not found"),
            _ => HttpResponse::InternalServerError().body(format!("Error: {:?}", err)),
        },
    }
}

pub async fn update_appointment(
    database: web::Data<Arc<Mutex<Database>>>,
    config: web::Data<AppConfig>,
    appointment_id: web::Path<AppointmentId>,
    update: web::Json<UpdateAppointment>,
) -> impl Responder {
    let db = match database.lock() {
        Ok(guard) => guard,
        Err(err) => {
            return HttpResponse::InternalServerError().body(format!("Lock error: {:?}", err))
        }
    };

    let mut appointment: AppointmentRecord = match db.read_appointment(&appointment_id.id).await {
        Ok(Some(appointment)) => appointment.into_appointment_record(),
        Ok(None) => return HttpResponse::NotFound().body("Appointment not found"),
        Err(err) => return HttpResponse::InternalServerError().body(format!("Error: {:?}", err)),
    };

    if let Some(start_time) = &update.start_time {
        appointment.start_time = *start_time;
        appointment.end_time = appointment.calculate_end_time();
    }
    if let Some(appointment_type) = &update.appointment_type {
        appointment.appointment_type = appointment_type.clone();
        appointment.end_time = appointment.calculate_end_time();
    }
    if let Some(doctor) = &update.doctor {
        if !config.is_valid_doctor(*doctor) {
            return HttpResponse::BadRequest().body(format!(
                "Doctor not found. The configured maximum doctor is {}.\nCount starts at 0",
                config.doctor_amount - 1
            ));
        }
        appointment.doctor = *doctor;
    }
    if let Some(room_nr) = &update.room_nr {
        if !config.is_valid_room(*room_nr) {
            return HttpResponse::BadRequest().body(format!(
                "Room not found. The configured maximum room is {}.\nCount starts at 0",
                config.room_amount - 1
            ));
        }
        appointment.room_nr = *room_nr;
    }

    let all_appointments = match db.read_all_appointments().await {
        Ok(appointments) => appointments,
        Err(err) => return HttpResponse::InternalServerError().body(format!("Error: {:?}", err)),
    };
    let all_appointments: Vec<AppointmentRecordWithPatient> = all_appointments
        .as_slice()
        .iter()
        .filter(|a| a.id.id.to_raw() != appointment_id.id)
        .cloned()
        .collect();

    match is_valid_timeframe(
        appointment.start_time,
        appointment.end_time,
        appointment.doctor,
        appointment.room_nr,
        &all_appointments,
        &config,
    )
    .await
    {
        Ok(_) => match db.update_appointment(&appointment_id.id, appointment).await {
            Ok(result) => HttpResponse::Ok().json(result),
            Err(err) => HttpResponse::InternalServerError().body(format!("Error: {:?}", err)),
        },
        Err(e) => HttpResponse::BadRequest().body(format!("Error: {:?}", e)),
    }
}

pub async fn read_appointment(
    database: web::Data<Arc<Mutex<Database>>>,
    appointment_id: web::Path<AppointmentId>,
) -> impl Responder {
    let db = match database.lock() {
        Ok(guard) => guard,
        Err(err) => {
            return HttpResponse::InternalServerError().body(format!("Lock error: {:?}", err))
        }
    };
    match db.read_appointment(&appointment_id.id).await {
        Ok(appointments) => HttpResponse::Ok().json(ApiResponse { data: appointments }),
        Err(err) => match err {
            DatabaseError::NothingFound => HttpResponse::NotFound().body("Appointment not found"),
            _ => HttpResponse::InternalServerError().body(format!("Error: {:?}", err)),
        },
    }
}
