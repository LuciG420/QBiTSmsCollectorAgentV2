use actix_web::{web, App, HttpServer, HttpResponse, Responder};

async fn mock_handler() -> impl Responder {
    HttpResponse::Ok().body("Mock response")
}

#[actix_web::main]
pub async fn run_mock_server() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/mock", web::get().to(mock_handler))
    })
    .bind("127.0.0.1:8081")?
    .run()
    .await
}
