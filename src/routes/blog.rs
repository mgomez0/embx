use actix_web::{get, web, HttpRequest, HttpResponse, Responder};
use tera::Tera;

#[get("/blog")]
pub async fn blog_list(tera: web::Data<Tera>) -> impl Responder {
    // Read the file containing blog post titles
    let html_files = vec![
        "day1.html",
        "evaluation_plan.html",
        "metapost.html", /* add more files as needed */
    ];

    //strip the .html extension and underscores
    let html_files = html_files
        .iter()
        .map(|x| x.replace("_", "").replace(".html", ""))
        .collect::<Vec<String>>();

    // Create a context with the list of blog posts
    let mut context = tera::Context::new();
    context.insert("html_files", &html_files);

    // Render the blog list template
    let rendered = tera.render("blog/blog_list.html", &context).unwrap();

    HttpResponse::Ok().body(rendered)
}

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
