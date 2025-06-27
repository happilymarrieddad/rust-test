use std::env;

use actix_web::{error, App, HttpServer, web, HttpResponse};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
// use std::{
//     cell::Cell,
//     sync::atomic::{AtomicUsize, Ordering},
//     sync::Arc,
// };

mod api;
mod types;
mod repository;

#[derive(Clone)]
struct AppState {
    pool: Pool<Postgres>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut db_url = env::var("OLD_WORLD_BUILDER_RUST_DB_URL").expect("OLD_WORLD_BUILDER_RUST_DB_URL env is required to run tests");
        db_url = String::from("postgres://postgres:postgres@localhost:5432/oldworld-test?connect_timeout=180&sslmode=disable");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(db_url.as_str())
        .await.expect("unable to get database connection");

    HttpServer::new(move || {
        let data = AppState {
            pool: pool.clone(),
        };

        let json_config = web::JsonConfig::default()
            .limit(4096)
            .error_handler(|err, _req| {
                // create custom error response
                error::InternalError::from_response(err, HttpResponse::Conflict().finish())
                    .into()
            });

        App::new()
            // config
            .app_data(web::Data::new(data.clone()))
            .app_data(json_config)

            // base routes
            .route("/login", web::post().to(api::auth::login::login))

            // v1 routes
            // users
            .route("/v1/users", web::get().to(api::v1::users::handlers::find))
            .route("/v1/users", web::post().to(api::v1::users::handlers::get))
            .route("/v1/users/{id}", web::get().to(api::v1::users::handlers::get))
            .route("/v1/users/{id}", web::put().to(api::v1::users::handlers::get))
            .route("/v1/users/{id}", web::delete().to(api::v1::users::handlers::get))
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
