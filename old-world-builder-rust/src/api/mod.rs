use actix_web::{get, post, web, App, HttpResponse, Responder};

pub fn init(app: App) -> App {
    app
        .service(hello)
        .service(echo)
        .route("/hey", web::get().to(manual_hello))
}