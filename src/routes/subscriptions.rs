use actix_web::{HttpResponse, web, Responder};

#[derive(serde::Deserialize)]
pub struct FormData {
    pub name: String,
    pub email: String
}

pub async fn health_check() -> impl Responder {
    HttpResponse::Ok().finish()
}

pub async fn subscribe(_form: web::Form<FormData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}