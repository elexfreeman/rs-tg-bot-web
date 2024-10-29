use actix_cors::Cors;
use actix_web::{get, http, post, web, App, HttpResponse, HttpServer, Responder, http::header};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("{}")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    println!("{}", req_body);
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let cors = Cors::default()
//            .allowed_origin("http://127.0.0.1:8088")
            //.allowed_origin("no-cors")
            .allow_any_origin()
            .allow_any_header()
            .allow_any_method()
            .expose_any_header()
            .supports_credentials()
            .max_age(3600);
        App::new()
            .wrap(cors)
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("0.0.0.0", 8088))?
    .run()
    .await
}
