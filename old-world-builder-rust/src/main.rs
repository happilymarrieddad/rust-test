use std::env;
use std::time::Duration;

use actix_web::{error, middleware::{from_fn}, web, App, HttpResponse, HttpServer};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use actix_cors::Cors;
use actix_extensible_rate_limit::{
    backend::{memory::InMemoryBackend, SimpleInputFunctionBuilder},
    RateLimiter,
};

mod api;
mod types;
mod repository;
mod middleware;

#[derive(Clone)]
struct AppState {
    pool: Pool<Postgres>,
    user_repo: repository::users::UserRepo,
    jwt_secret: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL env is required to run service");
    let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET env is required to run service");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(db_url.as_str())
        .await.expect("unable to get database connection");

    HttpServer::new(move || {
        let data = AppState {
            pool: pool.clone(),
            user_repo: repository::users::UserRepo::new(pool.clone()),
            jwt_secret: jwt_secret.clone(),
        };

        let json_config = web::JsonConfig::default()
            .limit(4096)
            .error_handler(|err, _req| {
                // create custom error response
                error::InternalError::from_response(err, HttpResponse::Conflict().finish())
                    .into()
            });

        let rate_limiter_backend = InMemoryBackend::builder().build();

        App::new()
            // config
            .app_data(web::Data::new(data.clone()))
            .app_data(json_config)
            .wrap(Cors::permissive())
            .wrap(
                RateLimiter::builder(
                    rate_limiter_backend.clone(),
                    SimpleInputFunctionBuilder::new(Duration::from_secs(60), 50)
                        .real_ip_key()
                        .build(),
                )
                .add_headers()
                .build(),
            )

            // base routes
            .route("/login", web::post().to(api::auth::login::login))

            // v1 routes
            // users
            .service(
                web::scope("/v1")
                    .wrap(from_fn(middleware::auth::verify_jwt))
                    .route("/users", web::get().to(api::v1::users::handlers::find))
                    .route("/users", web::post().to(api::v1::users::handlers::create))
                    .route("/users/{id}", web::get().to(api::v1::users::handlers::get))
                    .route("/users/{id}", web::put().to(api::v1::users::handlers::update))
                    .route("/users/{id}", web::delete().to(api::v1::users::handlers::delete))
            )
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
