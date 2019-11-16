use actix_files::Files;
use actix_web::{web, App, HttpRequest, HttpServer, Responder};

fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello, {}!\n", &name)
}

fn main() {
    HttpServer::new(|| {
        App::new()
            .route("/greet/{name}", web::get().to(greet))
            // For all other paths, redirect to static file
            .service(Files::new("/", "static/").index_file("index.html"))
    })
    .bind("127.0.0.1:8000")
    .expect("Can not bind to port 8000")
    .run()
    .unwrap();
}
