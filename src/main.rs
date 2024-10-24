use actix_files::Files;
use actix_files::NamedFile;
use actix_web::HttpRequest;
use std::path::PathBuf;

async fn index(req: HttpRequest) -> actix_web::Result<NamedFile> {
    let path: PathBuf = req.match_info().query("filename").parse().unwrap();
    Ok(NamedFile::open(path)?)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{web, App, HttpServer};
    let app_port = 8088;

    println!("Server start at 8088");
    HttpServer::new(|| App::new().service(
        Files::new("/", "./static")
                .show_files_listing()
                .index_file("index.html")
                .use_last_modified(true)
        .prefer_utf8(true)))
        .bind(format!("[::1]:{}", app_port))?
        .run()
        .await
}
