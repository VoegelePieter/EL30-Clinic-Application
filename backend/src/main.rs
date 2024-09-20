use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use backend::appointment_endpoints::{
    create_appointment, delete_appointment, mass_reschedule_doctor, read_all_appointments_handler,
    read_appointment, update_appointment,
};
use backend::config::AppConfig;
use backend::config_endpoints::{get_doctor_amount, get_room_amount};
use backend::db::db::Database;
use backend::patient_endpoints::{
    create_patient, delete_patient, read_all_patients, read_patient, update_patient,
};
use std::sync::{Arc, Mutex};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let config = AppConfig::new();

    let port = config.port;

    let database = Arc::new(Mutex::new(Database::new()));
    database
        .lock()
        .unwrap()
        .initiate_db(config.clone())
        .await
        .expect(
            "Couldn't initiate database. Make sure the server is running and configured correctly.",
        );

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new()
            .wrap(cors)
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
                        web::resource("/appointment/mass_reschedule")
                            .route(web::post().to(mass_reschedule_doctor)),
                    )
                    .service(
                        web::resource("/appointment/{id}")
                            .route(web::get().to(read_appointment))
                            .route(web::put().to(update_appointment))
                            .route(web::delete().to(delete_appointment)),
                    )
                    .service(
                        web::resource("/config/doctor_amount")
                            .route(web::get().to(get_doctor_amount)),
                    )
                    .service(
                        web::resource("/config/room_amount").route(web::get().to(get_room_amount)),
                    ),
            )
    })
    .bind(format!("127.0.0.1:{}", port))?
    .run()
    .await
}
