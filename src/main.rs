use actix_files::Files;
use actix_web::{middleware::Logger, web, App, HttpServer};

mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // access logs are printed with the INFO level so ensure it is enabled by default
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(routes::load_templates()))
            .service(Files::new("/static", "src/style").show_files_listing())
            .service(Files::new("/assets", "src/assets").show_files_listing())
            .service(routes::home::home)
            .service(routes::blog::blog)
            .service(routes::blog::blog_list)
            .service(routes::contact::contact)
    })
    .bind("0.0.0.0:3000")?
    .run()
    .await
}
