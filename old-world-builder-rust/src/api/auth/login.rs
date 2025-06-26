use actix_web::{post, HttpResponse, Responder};

pub async fn login() -> impl Responder {
    HttpResponse::Ok().body("success")
}