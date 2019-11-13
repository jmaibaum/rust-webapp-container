use actix_files::NamedFile;
use actix_web::{web, App, HttpRequest, HttpServer, Responder, Result};
use std::path::PathBuf;

fn index(req: HttpRequest) -> Result<NamedFile> {
    let path: PathBuf = req.match_info().query("filename").parse().unwrap();
    Ok(NamedFile::open(path)?)
}

fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello, {}!\n", &name)
}

fn main() {
    HttpServer::new(|| {
        App::new()
            .route("/{filename:.*}", web::get().to(index))
            .route("/{name}", web::get().to(greet))
    })
    .bind("127.0.0.1:8000")
    .expect("Can not bind to port 8000")
    .run()
    .unwrap();
}
