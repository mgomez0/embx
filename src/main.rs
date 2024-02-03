use actix_files::Files;
use actix_web::{web, App, HttpServer};

mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(routes::load_templates()))
            .service(Files::new("/static", "src/style").show_files_listing())
            .service(routes::home::home)
            .service(routes::blog::blog)
            .service(routes::blog::blog_list)
    })
    .bind("127.0.0.1:3000")?
    .run()
    .await
}
