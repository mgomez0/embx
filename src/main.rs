use actix_files::Files;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use tera::Tera;

mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(routes::load_templates()))
            .service(Files::new("/static", "style").show_files_listing())
            .service(routes::home::home)
            .service(routes::blog::blog)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
