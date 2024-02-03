pub mod blog;
pub mod home;
use tera::Tera;

pub fn load_templates() -> Tera {
    // Load Tera templates
    Tera::new("src/templates/**/*.html").expect("Failed to parse templates")
}
