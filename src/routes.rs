use crate::handlers::event_attendance::{get_event_attendees, save_event_attendance};
use crate::models::EventAttendance;
use actix_web::{web, HttpResponse, Responder};

pub async fn post_event_attendance(info: web::Json<EventAttendance>) -> impl Responder {
    match save_event_attendance(info.into_inner()).await {
        Ok(_) => HttpResponse::Ok().body("Data saved successfully"),
        Err(_) => HttpResponse::InternalServerError().body("Failed to save data"),
    }
}

pub async fn fetch_event_attendees() -> impl Responder {
    match get_event_attendees().await {
        Ok(attendees) => HttpResponse::Ok().json(attendees),
        Err(e) => {
            eprintln!("error = {}", e);
            HttpResponse::InternalServerError().body("Failed to fetch data, err")}

    }
}
