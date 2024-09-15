mod appointment_endpoints;
mod config;
mod db;
mod patient_endpoints;
mod types;
mod util;

use crate::appointment_endpoints::{
    create_appointment, delete_appointment, read_appointment, update_appointment,
};
use crate::patient_endpoints::{
    create_patient, delete_patient, read_all_patients, read_patient, update_patient,
};
use actix_web::{web, App, HttpServer};
use appointment_endpoints::read_all_appointments_handler;
use config::AppConfig;
use db::db::Database;
use std::sync::{Arc, Mutex};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let config = AppConfig::new();

    let port = config.port;

    let database = Arc::new(Mutex::new(Database::new()));
    database.lock().unwrap().initiate_db().await.expect(
        "Couldn't initiate database. Make sure the server is running and configured correctly.",
    );

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(database.clone()))
            .app_data(web::Data::new(config.clone()))
            .service(
                web::scope("/api")
                    .service(
                        web::resource("/patient")
                            .route(web::post().to(create_patient))
                            .route(web::get().to(read_all_patients)),
                    )
                    .service(
                        web::resource("/patient/{id}")
                            .route(web::get().to(read_patient))
                            .route(web::put().to(update_patient))
                            .route(web::delete().to(delete_patient)),
                    )
                    .service(
                        web::resource("/appointment")
                            .route(web::post().to(create_appointment))
                            .route(web::get().to(read_all_appointments_handler)),
                    )
                    .service(
                        web::resource("/appointment/{id}")
                            .route(web::get().to(read_appointment))
                            .route(web::put().to(update_appointment))
                            .route(web::delete().to(delete_appointment)),
                    ),
            )
    })
    .bind(format!("127.0.0.1:{}", port))?
    .run()
    .await
}
