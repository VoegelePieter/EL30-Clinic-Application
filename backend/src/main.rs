mod db;
mod patient_endpoints;
mod types;

use crate::patient_endpoints::{
    create_patient, delete_patient, read_all_patients, read_patient, update_patient,
};
use actix_web::{web, App, HttpServer};
use db::db::Database;
use std::sync::{Arc, Mutex};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let database = Arc::new(Mutex::new(Database::new()));
    database.lock().unwrap().initiate_db().await.expect(
        "Couldn't initate database. Make sure the server is running and configured correctly.",
    );

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(database.clone()))
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
                    ),
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
