use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;
use serde_json::json;

use crate::repository::users::{UserRepoFindOpts};
use crate::AppState;
use crate::types::users::{create_user, update_user};

#[derive(Deserialize)]
pub struct FindOpts {
    limit: i64,
    offset: i64,
    email: String,
}

pub async fn get(state: web::Data<AppState>, path: web::Path<i64>) -> impl Responder {
    let repo = state.user_repo.clone();

    let id = path.into_inner();
    
    let result = repo.get(id).await;
    if result.is_err() {
       return HttpResponse::BadRequest().json(json!({
            "status": 400,
            "message": result.err().unwrap().to_string(),
        }));
    }

    HttpResponse::Ok().json(result.unwrap())
}

pub async fn find(state: web::Data<AppState>, opts: web::Query<FindOpts>) -> impl Responder {
    let repo = state.user_repo.clone();

    let result = repo
        .find(UserRepoFindOpts{
            limit: opts.limit, offset: opts.offset,
            id: 0, email: opts.email.clone(),
        })
        .await;
    if result.is_err() {
       return HttpResponse::BadRequest().json(json!({
            "status": 400,
            "message": result.err().unwrap().to_string(),
        }));
    }

    HttpResponse::Ok().json(result.unwrap())
}

pub async fn create(state: web::Data<AppState>, body: web::Json<create_user>) -> impl Responder {
    let repo = state.user_repo.clone();

    let result = repo.create(body.0).await;
    if result.is_err() {
       return HttpResponse::BadRequest().json(json!({
            "status": 400,
            "message": result.err().unwrap().to_string(),
        }));
    }

    HttpResponse::Created().json(result.unwrap())
}

pub async fn update(state: web::Data<AppState>, path: web::Path<i64>, body: web::Json<update_user>) -> impl Responder {
    let id = path.into_inner();
    let repo = state.user_repo.clone();

    let result = repo.update(id, body.0).await;
    if result.is_err() {
       return HttpResponse::BadRequest().json(json!({
            "status": 400,
            "message": result.err().unwrap().to_string(),
        }));
    }

    HttpResponse::Ok().json(result.unwrap())
}

pub async fn delete(state: web::Data<AppState>, path: web::Path<i64>) -> impl Responder {
    // let id = path.into_inner();
    // let repo = state.user_repo.clone();

    // let result = repo.delete(id).await;
    // if result.is_err() {
    //    return HttpResponse::BadRequest().json(json!({
    //         "status": 400,
    //         "message": result.err().unwrap().to_string(),
    //     }));
    // }

    // HttpResponse::Ok().json(result.unwrap())
    HttpResponse::NotImplemented()
}
