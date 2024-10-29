use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn hello() -> impl Responder {
        HttpResponse::Ok().body("{}")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
        println!("{}",req_body);
        HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
        HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
        HttpServer::new(|| {
                    App::new()
                    .service(hello)
                    .service(echo)
                    .route("/hey", web::get().to(manual_hello))
        })
        .bind(("0.0.0.0", 8088))?
        .run()
        .await
}
