// src/controllers/index.rs

use actix_web::{get, HttpResponse};

#[get("/")]
pub async fn index() -> HttpResponse {
    HttpResponse::Ok().body("Ok")
}


#[get("/status")]
pub async fn status() -> HttpResponse {
    HttpResponse::Ok().body("Alright")
}