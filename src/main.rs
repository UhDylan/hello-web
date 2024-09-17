use actix_files as fs;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};


#[actix_web::main]
async fn main() -> std::io::Result<()> {
        HttpServer::new(|| {
        App::new()
            .service(fs::Files::new("/", "./html").index_file("index.html"))
            .route("/hello", web::get().to(hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

async fn hello() -> impl Responder {
    HttpResponse::Ok().body("hello")
}
 
