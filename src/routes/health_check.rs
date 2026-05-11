use actix_web::{App, HttpResponse, HttpServer, web};

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}