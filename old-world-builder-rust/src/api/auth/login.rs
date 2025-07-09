use std::time::SystemTime;

use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use serde_json::json;
use crate::AppState;
use bcrypt::{verify};
use jsonwebtoken::{EncodingKey, Header};

#[derive(Deserialize)]
pub struct login_data {
    email: String,
    password: String,
}

#[derive(Serialize, Deserialize)]
pub struct claims {
    pub sub: i64,
    pub role: String,
    pub exp: u64,
}

pub async fn login(state: web::Data<AppState>, data: web::Json<login_data>) -> impl Responder {
    let repo = state.user_repo.clone();

    let user_get_by_id_result = repo.find_by_email(data.email.clone()).await;
    if user_get_by_id_result.is_err() {
       return HttpResponse::BadRequest().json(json!({
            "status": 400,
            "message": user_get_by_id_result.err().unwrap().to_string(),
        }));
    }

    let mut user = user_get_by_id_result.unwrap();

    let password_check_result = user.verify_password(data.password.clone());
    if !password_check_result {
        return HttpResponse::Unauthorized().json(json!({
            "error": String::from("unauthorized"),
        }));
    }

    let c = claims{
        sub: user.id,
        role: String::from("user"),
        exp: SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs()
            + 4 * 60 * 60,
    };

    let token = jsonwebtoken::encode(
        &Header::default(),
        &c,
        &EncodingKey::from_secret(state.jwt_secret.as_bytes()),
    ).unwrap();

    user.password = String::from(""); // sanitize the password
    HttpResponse::Ok().json(json!({
        "status": "success",
        "token": token,
        "user": user,
    }))
}