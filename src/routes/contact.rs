use actix_web::{get, web, HttpResponse, Responder};
use tera::Tera;

#[get("/contact")]
pub async fn contact(tera: web::Data<Tera>) -> impl Responder {
    let context = tera::Context::new();
    let rendered = tera.render("contact.html", &context).unwrap();
    HttpResponse::Ok().body(rendered)
}
