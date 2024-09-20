use actix_web::{web, HttpResponse, Responder};

use crate::config::AppConfig;

pub async fn get_doctor_amount(config: web::Data<AppConfig>) -> impl Responder {
    HttpResponse::Ok().body(format!("{}", config.doctor_amount - 1))
}

pub async fn get_room_amount(config: web::Data<AppConfig>) -> impl Responder {
    HttpResponse::Ok().body(format!("{}", config.room_amount - 1))
}
