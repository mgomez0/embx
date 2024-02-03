use actix_web::{get, web, HttpResponse, Responder};
use tera::Tera;

#[get("/")]
pub async fn home(tera: web::Data<Tera>) -> impl Responder {
    let context = tera::Context::new();
    let rendered = tera.render("index.html", &context).unwrap();
    HttpResponse::Ok().body(rendered)
}
