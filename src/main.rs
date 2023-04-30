use actix_http::body::BoxBody;
use actix_web::dev::{ServiceFactory, ServiceRequest, ServiceResponse};
use actix_web::Error;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

pub fn create_app() -> App<
    impl ServiceFactory<
        ServiceRequest,
        Response = ServiceResponse<BoxBody>,
        Config = (),
        InitError = (),
        Error = Error,
    >,
> {
    App::new()
        .service(hello)
        .service(echo)
        .route("/hey", web::get().to(manual_hello))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(create_app)
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}

mod test;
