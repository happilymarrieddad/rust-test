use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;

use crate::AppState;
use crate::types::users::{create_user, update_user};

#[derive(Deserialize)]
pub struct FindOpts {
    limit: i64,
    offset: i64,
}

pub async fn get(path: web::Path<(u32)>) -> impl Responder {
    let id = path.into_inner();
    HttpResponse::NotImplemented()
}

pub async fn find(opts: web::Query<FindOpts>) -> impl Responder {
    HttpResponse::NotImplemented()
}

pub async fn create(data: web::Data<AppState>, info: web::Json<create_user>) -> impl Responder {
    HttpResponse::NotImplemented()
}

pub async fn update(path: web::Path<(u32)>, data: web::Data<AppState>, info: web::Json<update_user>) -> impl Responder {
    let id = path.into_inner();
    HttpResponse::NotImplemented()
}

pub async fn delete(path: web::Path<(u32)>) -> impl Responder {
    let id = path.into_inner();
    HttpResponse::NotImplemented()
}