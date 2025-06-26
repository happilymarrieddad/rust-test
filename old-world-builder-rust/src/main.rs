use actix_web::{App, HttpServer, web};

mod api;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            // .route("/1", web::service(
            //     web::route("/",web::get().to(api::v1::handlers::manual_hello))
            // ))
            .service(api::v1::handlers::hello)
            .service(api::v1::handlers::echo)
            .route("/hey", web::get().to(api::v1::handlers::manual_hello))
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
