// src/templates/engine.rs
use tera::Tera;

pub fn init() -> Tera {
    let mut tera = match Tera::new("templates/**/*.html") {
        Ok(t)  => t,
        Err(e) => {
            eprintln!("❌  Erreur chargement templates : {}", e);
            std::process::exit(1);
        }
    };

    tera.autoescape_on(vec![]);
    tera
}
