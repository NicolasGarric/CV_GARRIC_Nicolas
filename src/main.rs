// src/main.rs
mod components;
mod data;
mod pdf;
mod templates;
mod utils;

use actix_files::Files;
use actix_web::{middleware, web, App, HttpServer};
use log::info;

use crate::data::cv::CvData;
use crate::templates::engine;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(
        env_logger::Env::default().default_filter_or("info")
    );

    let cv_data = web::Data::new(CvData::load());
    let tera    = web::Data::new(engine::init());

    let bind_addr = "127.0.0.1:8080";
    info!("🚀  Serveur démarré → http://{}", bind_addr);
    info!("📄  Export PDF     → http://{}/export-pdf", bind_addr);

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .app_data(cv_data.clone())
            .app_data(tera.clone())
            .configure(routes::register)
            .service(Files::new("/static", "static"))
    })
    .bind(bind_addr)?
    .run()
    .await
}

mod routes {
    use actix_web::web;
    use crate::templates::pages;
    use crate::pdf;

    pub fn register(cfg: &mut web::ServiceConfig) {
        // CV Français
        cfg.route("/",           web::get().to(pages::cv::render));
        cfg.route("/export-pdf", web::get().to(pdf::export));

        // CV Anglais
        cfg.route("/en",           web::get().to(pages::cv::render_en));
        cfg.route("/en/export-pdf", web::get().to(pdf::export_en));
    }
}
