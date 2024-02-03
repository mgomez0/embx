use actix_web::{get, web, HttpRequest, HttpResponse, Responder};
use tera::Tera;

#[get("/blog/{slug}")]
pub async fn blog(req: HttpRequest, tera: web::Data<Tera>) -> impl Responder {
    // Extract the blog post slug from the request
    let slug = req.match_info().get("slug").unwrap_or("unknown");

    // Render the blog post template using Tera
    let template_name = format!("blog/{}.html", slug);
    let context = tera::Context::new();
    let rendered = tera
        .render(&template_name, &context)
        .unwrap_or_else(|_| String::from("Error rendering template"));

    HttpResponse::Ok().body(rendered)
}
