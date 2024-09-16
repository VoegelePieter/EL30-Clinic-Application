use std::sync::{Arc, Mutex};

use actix_web::{test, web, App};
use backend::appointment_endpoints::{
    create_appointment, delete_appointment, mass_reschedule_doctor, read_all_appointments_handler,
    read_appointment, update_appointment,
};
use backend::config::AppConfig;
use backend::db::db::Database;
use backend::patient_endpoints::{
    create_patient, delete_patient, read_all_patients, read_patient, update_patient,
};
use chrono::NaiveTime;

async fn get_test_config() -> AppConfig {
    AppConfig {
        port: 8080,
        doctor_amount: 5,
        room_amount: 10,
        namespace: "test".to_string(),
        database: "test".to_string(),
        database_url: "127.0.0.1:8000".to_string(),
        opening_time: NaiveTime::from_hms_opt(8, 0, 0).unwrap(),
        closing_time: NaiveTime::from_hms_opt(17, 0, 0).unwrap(),
        break_time: NaiveTime::from_hms_opt(12, 0, 0).unwrap(),
    }
}

async fn mock_db() -> Arc<Mutex<Database>> {
    let db = Database::new();
    db.initiate_db(get_test_config().await).await.unwrap();
    Arc::new(Mutex::new(db))
}

#[actix_rt::test]
async fn test_endpoint_create_patient() {
    // Initialize the configuration and database
    let config = get_test_config().await;
    let database = mock_db().await;

    // Initialize the Actix web application
    let mut app = test::init_service(
        App::new()
            .app_data(web::Data::new(database.clone()))
            .app_data(web::Data::new(config.clone()))
            .service(
                web::scope("/api")
                    .service(web::resource("/patient").route(web::post().to(create_patient))),
            ),
    )
    .await;

    // Create a test request to the /api/patient endpoint
    let req = test::TestRequest::post()
        .uri("/api/patient")
        .set_json(&serde_json::json!({
            "name": "John Doe",
            "phone_number": "1234567890"
        }))
        .to_request();

    // Call the service and get the response
    let resp = test::call_service(&mut app, req).await;

    // Assert that the response status is successful
    assert!(resp.status().is_success());
}

#[actix_rt::test]
async fn test_endpoint_read_patient_by_id() {
    // Initialize the configuration and database
    let config = get_test_config().await;
    let database = mock_db().await;

    // Initialize the Actix web application
    let mut app = test::init_service(
        App::new()
            .app_data(web::Data::new(database.clone()))
            .app_data(web::Data::new(config.clone()))
            .service(
                web::scope("/api")
                    .service(web::resource("/patient").route(web::post().to(create_patient)))
                    .service(web::resource("/patient/{id}").route(web::get().to(read_patient))),
            ),
    )
    .await;

    // Create a test request to the /api/patient endpoint
    let req = test::TestRequest::get()
        .uri("/api/patient/5ouz3rzkdyje2lgadtcm")
        .to_request();

    // Call the service and get the response
    let resp = test::call_service(&mut app, req).await;

    // Assert that the response status is successful
    assert!(resp.status().is_client_error());
}

#[actix_rt::test]
async fn test_endpoint_read_all_patients() {
    // Initialize the configuration and database
    let config = get_test_config().await;
    let database = mock_db().await;

    // Initialize the Actix web application
    let mut app = test::init_service(
        App::new()
            .app_data(web::Data::new(database.clone()))
            .app_data(web::Data::new(config.clone()))
            .service(
                web::scope("/api")
                    .service(web::resource("/patient").route(web::get().to(read_all_patients))),
            ),
    )
    .await;

    // Create a test request to the /api/patient endpoint
    let req = test::TestRequest::get().uri("/api/patient").to_request();

    // Call the service and get the response
    let resp = test::call_service(&mut app, req).await;

    // Assert that the response status is successful
    assert!(resp.status().is_success());
}

#[actix_rt::test]
async fn test_endpoint_delete_patient() {
    // Initialize the configuration and database
    let config = get_test_config().await;
    let database = mock_db().await;

    // Initialize the Actix web application
    let mut app =
        test::init_service(
            App::new()
                .app_data(web::Data::new(database.clone()))
                .app_data(web::Data::new(config.clone()))
                .service(web::scope("/api").service(
                    web::resource("/patient/{id}").route(web::delete().to(delete_patient)),
                )),
        )
        .await;

    // Create a test request to the /api/patient endpoint
    let req = test::TestRequest::delete()
        .uri("/api/patient/invalid")
        .to_request();

    // Call the service and get the response
    let resp = test::call_service(&mut app, req).await;

    // Assert that the response status is successful
    assert!(resp.status().is_client_error());
}

#[actix_rt::test]
async fn test_endpoint_update_patient() {
    // Initialize the configuration and database
    let config = get_test_config().await;
    let database = mock_db().await;

    // Initialize the Actix web application
    let mut app = test::init_service(
        App::new()
            .app_data(web::Data::new(database.clone()))
            .app_data(web::Data::new(config.clone()))
            .service(
                web::scope("/api")
                    .service(web::resource("/patient/{id}").route(web::get().to(update_patient))),
            ),
    )
    .await;

    // Create a test request to the /api/patient endpoint
    let req = test::TestRequest::put()
        .uri("/api/patient/invalid")
        .to_request();

    // Call the service and get the response
    let resp = test::call_service(&mut app, req).await;

    // Assert that the response status is successful
    assert!(resp.status().is_client_error());
}

#[actix_rt::test]
async fn test_endpoint_create_appointment() {
    // Initialize the configuration and database
    let config = get_test_config().await;
    let database = mock_db().await;

    // Initialize the Actix web application
    let mut app =
        test::init_service(
            App::new()
                .app_data(web::Data::new(database.clone()))
                .app_data(web::Data::new(config.clone()))
                .service(web::scope("/api").service(
                    web::resource("/appointment").route(web::post().to(create_appointment)),
                )),
        )
        .await;

    // Create a test request to the /api/patient endpoint
    let req = test::TestRequest::post()
        .uri("/api/appointment")
        .set_json(&serde_json::json!({
            "start_time": "2021-01-01T08:00:00",
            "appointment_type": "quick_checkup",
            "patient_id": "some_id",
            "doctor": 5,
            "room_nr": 1,
        }))
        .to_request();

    // Call the service and get the response
    let resp = test::call_service(&mut app, req).await;

    // Assert that the response status is successful
    assert!(resp.status().is_client_error());
}

#[actix_rt::test]
async fn test_endpoint_delete_appointment() {
    // Initialize the configuration and database
    let config = get_test_config().await;
    let database = mock_db().await;

    // Initialize the Actix web application
    let mut app = test::init_service(
        App::new()
            .app_data(web::Data::new(database.clone()))
            .app_data(web::Data::new(config.clone()))
            .service(web::scope("/api").service(
                web::resource("/appointment/{id}").route(web::delete().to(delete_appointment)),
            )),
    )
    .await;

    // Create a test request to the /api/patient endpoint
    let req = test::TestRequest::delete()
        .uri("/api/appointment/some_id")
        .to_request();

    // Call the service and get the response
    let resp = test::call_service(&mut app, req).await;

    // Assert that the response status is successful
    assert!(resp.status().is_client_error());
}

#[actix_rt::test]
async fn test_endpoint_mass_reschedule() {
    // Initialize the configuration and database
    let config = get_test_config().await;
    let database = mock_db().await;

    // Initialize the Actix web application
    let mut app = test::init_service(
        App::new()
            .app_data(web::Data::new(database.clone()))
            .app_data(web::Data::new(config.clone()))
            .service(
                web::scope("/api").service(
                    web::resource("/appointment/mass_reschedule")
                        .route(web::post().to(mass_reschedule_doctor)),
                ),
            ),
    )
    .await;

    // Create a test request to the /api/patient endpoint
    let req = test::TestRequest::post()
        .uri("/api/appointment/mass_reschedule")
        .set_json(&serde_json::json!({
            "doctor_id": 1,
            "start_date": "2021-01-01",
            "end_date": "2021-01-31"
        }))
        .to_request();

    // Call the service and get the response
    let resp = test::call_service(&mut app, req).await;

    // Assert that the response status is successful
    assert!(resp.status().is_success());
}

#[actix_rt::test]
async fn test_endpoint_read_all_appointments() {
    // Initialize the configuration and database
    let config = get_test_config().await;
    let database = mock_db().await;

    // Initialize the Actix web application
    let mut app = test::init_service(
        App::new()
            .app_data(web::Data::new(database.clone()))
            .app_data(web::Data::new(config.clone()))
            .service(web::scope("/api").service(
                web::resource("/appointment").route(web::get().to(read_all_appointments_handler)),
            )),
    )
    .await;

    // Create a test request to the /api/patient endpoint
    let req = test::TestRequest::get()
        .uri("/api/appointment")
        .to_request();

    // Call the service and get the response
    let resp = test::call_service(&mut app, req).await;

    // Assert that the response status is successful
    assert!(resp.status().is_success());
}

#[actix_rt::test]
async fn test_endpoint_read_appointment_by_id() {
    // Initialize the configuration and database
    let config = get_test_config().await;
    let database = mock_db().await;

    // Initialize the Actix web application
    let mut app = test::init_service(
        App::new()
            .app_data(web::Data::new(database.clone()))
            .app_data(web::Data::new(config.clone()))
            .service(web::scope("/api").service(
                web::resource("/appointment/{id}").route(web::get().to(read_appointment)),
            )),
    )
    .await;

    // Create a test request to the /api/patient endpoint
    let req = test::TestRequest::get()
        .uri("/api/appointment/some_id")
        .to_request();

    // Call the service and get the response
    let resp = test::call_service(&mut app, req).await;

    // Assert that the response status is successful
    assert!(resp.status().is_client_error());
}

#[actix_rt::test]
async fn test_endpoint_update_appointment() {
    // Initialize the configuration and database
    let config = get_test_config().await;
    let database = mock_db().await;

    // Initialize the Actix web application
    let mut app = test::init_service(
        App::new()
            .app_data(web::Data::new(database.clone()))
            .app_data(web::Data::new(config.clone()))
            .service(web::scope("/api").service(
                web::resource("/appointment/{id}").route(web::put().to(update_appointment)),
            )),
    )
    .await;

    // Create a test request to the /api/patient endpoint
    let req = test::TestRequest::put()
        .uri("/api/patient/some_id")
        .set_json(&serde_json::json!({
            "doctor": 0,
        }))
        .to_request();

    // Call the service and get the response
    let resp = test::call_service(&mut app, req).await;

    // Assert that the response status is successful
    assert!(resp.status().is_client_error());
}
