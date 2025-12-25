use actix_web::{App, HttpResponse, HttpServer, Responder, web};

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

async fn after_start() {
    tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
    println!("Server started on port 8080");
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    tokio::spawn(after_start());
    HttpServer::new(|| App::new().route("/hey", web::get().to(manual_hello)))
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}
