use std::sync::{Arc, Mutex};

use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;

use crate::db::{
    db::Database,
    types::{DatabaseError, Patient},
};
use crate::types::ApiResponse;

// Patient Types
#[derive(Deserialize)]
pub struct PatientId {
    id: String,
}
#[derive(Deserialize)]
pub struct UpdatePatient {
    name: Option<String>,
    phone_number: Option<String>,
    insurance_number: Option<String>,
}

// Endpoints
pub async fn read_all_patients(database: web::Data<Arc<Mutex<Database>>>) -> impl Responder {
    let db = match database.lock() {
        Ok(guard) => guard,
        Err(err) => {
            return HttpResponse::InternalServerError().body(format!("Lock error: {:?}", err))
        }
    };

    match db.read_all_patients().await {
        Ok(patients) => HttpResponse::Ok().json(ApiResponse { data: patients }),
        Err(err) => HttpResponse::InternalServerError().body(format!("Error: {:?}", err)),
    }
}

pub async fn create_patient(
    database: web::Data<Arc<Mutex<Database>>>,
    patient: web::Json<Patient>,
) -> impl Responder {
    let db = match database.lock() {
        Ok(guard) => guard,
        Err(err) => {
            return HttpResponse::InternalServerError().body(format!("Lock error: {:?}", err))
        }
    };

    match db.create_patient(patient.into_inner()).await {
        Ok(result) => HttpResponse::Ok().json(ApiResponse { data: result }),
        Err(err) => HttpResponse::InternalServerError().body(format!("Error: {:?}", err)),
    }
}

pub async fn delete_patient(
    database: web::Data<Arc<Mutex<Database>>>,
    patient_id: web::Path<PatientId>,
) -> impl Responder {
    let db = match database.lock() {
        Ok(guard) => guard,
        Err(err) => {
            return HttpResponse::InternalServerError().body(format!("Lock error: {:?}", err))
        }
    };
    match db.delete_patient(&patient_id.id).await {
        Ok(patients) => HttpResponse::Ok().json(ApiResponse { data: patients }),
        Err(err) => match err {
            DatabaseError::NothingFound => HttpResponse::NotFound().body("Patient not found"),
            _ => HttpResponse::InternalServerError().body(format!("Error: {:?}", err)),
        },
    }
}

pub async fn update_patient(
    database: web::Data<Arc<Mutex<Database>>>,
    patient_id: web::Path<PatientId>,
    update: web::Json<UpdatePatient>,
) -> impl Responder {
    let db = match database.lock() {
        Ok(guard) => guard,
        Err(err) => {
            return HttpResponse::InternalServerError().body(format!("Lock error: {:?}", err))
        }
    };
    let mut patient = match db.read_patient(&patient_id.id).await {
        Ok(patient) => patient,
        Err(err) => match err {
            DatabaseError::NothingFound => {
                return HttpResponse::NotFound().body("Patient not found")
            }
            _ => return HttpResponse::InternalServerError().body(format!("Error: {:?}", err)),
        },
    };

    if let Some(name) = &update.name {
        patient.name = name.clone();
    }
    if let Some(phone_number) = &update.phone_number {
        patient.phone_number = phone_number.clone();
    }
    if let Some(insurance_number) = &update.insurance_number {
        patient.insurance_number = Some(insurance_number.clone());
    }

    match db.update_patient(&patient_id.id, patient).await {
        Ok(result) => HttpResponse::Ok().json(ApiResponse { data: result }),
        Err(err) => HttpResponse::InternalServerError().body(format!("Error: {:?}", err)),
    }
}

pub async fn read_patient(
    database: web::Data<Arc<Mutex<Database>>>,
    patient_id: web::Path<PatientId>,
) -> impl Responder {
    let db = match database.lock() {
        Ok(guard) => guard,
        Err(err) => {
            return HttpResponse::InternalServerError().body(format!("Lock error: {:?}", err))
        }
    };
    match db.read_patient(&patient_id.id).await {
        Ok(patients) => HttpResponse::Ok().json(ApiResponse { data: patients }),
        Err(err) => match err {
            DatabaseError::NothingFound => HttpResponse::NotFound().body("Patient not found"),
            _ => HttpResponse::InternalServerError().body(format!("Error: {:?}", err)),
        },
    }
}
