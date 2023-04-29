use actix_web::{App, HttpServer}; //add
use handlers::{send_otp, verify_otp}; //add
mod handlers;
mod model;
mod services;

//add
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || App::new().service(send_otp).service(verify_otp))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}