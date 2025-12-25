use actix_web::{App, HttpResponse, HttpServer, Responder, web};

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/hey", web::get().to(manual_hello)))
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}
