use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use routes::{fetch_event_attendees, post_event_attendance};
use std::env;
mod db;
mod handlers;
mod models;
mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let port = env::var("PORT").expect("PORT must be set").parse::<u16>().expect("PORT must be a number");

    HttpServer::new(|| {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();

        App::new()
            .wrap(cors)
            .service(
                web::resource("/confirm-attendance").route(web::post().to(post_event_attendance)),
            )
            .service(web::resource("/attendees").route(web::get().to(fetch_event_attendees)))
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
