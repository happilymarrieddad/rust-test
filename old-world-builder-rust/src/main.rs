use actix_web::{error, App, HttpServer, web, HttpResponse};
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
    // local_count: Cell<usize>,
    // global_count: Arc<AtomicUsize>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let data = AppState {
            // local_count: Cell::new(0),
            // global_count: Arc::new(AtomicUsize::new(0)),
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
