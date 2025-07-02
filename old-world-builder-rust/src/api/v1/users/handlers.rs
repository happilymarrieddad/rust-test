use actix_web::http::header::ContentType;
use actix_web::mime::APPLICATION_JSON;
use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;

use crate::repository::users::UserRepo;
use crate::AppState;
use crate::types::users::{self, create_user, update_user, user};

#[derive(Deserialize)]
pub struct FindOpts {
    limit: i64,
    offset: i64,
}

pub async fn get(path: web::Path<u32>) -> impl Responder {
    let id = path.into_inner();
    HttpResponse::NotImplemented()
}

pub async fn find(opts: web::Query<FindOpts>) -> impl Responder {
    HttpResponse::NotImplemented()
}

pub async fn create(data: web::Data<AppState>, new_user: web::Json<create_user>) -> impl Responder {
    let repo = UserRepo::new(data.pool.clone());

    match repo.create(create_user{
        first_name: new_user.first_name.clone(),
        last_name: new_user.last_name.clone(),
        email: new_user.email.clone(),
        password: new_user.password.clone(),
        password_confirm: new_user.password_confirm.clone(),
    }).await {
        Err(err) => {
            HttpResponse::InternalServerError()
        }
        Ok(created_user) => {
            // TODO: return the created user
            HttpResponse::Created() //.json(created_user)
        }
    }
}

pub async fn update(path: web::Path<u32>, data: web::Data<AppState>, info: web::Json<update_user>) -> impl Responder {
    let id = path.into_inner();
    HttpResponse::NotImplemented()
}

pub async fn delete(path: web::Path<u32>) -> impl Responder {
    let id = path.into_inner();
    HttpResponse::NotImplemented()
}
