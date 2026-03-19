// src/templates/pages/cv.rs
use actix_web::{web, HttpResponse, Result};
use tera::{Context, Tera};
use crate::data::cv::CvData;

pub async fn render(
    tera:    web::Data<Tera>,
    cv_data: web::Data<CvData>,
) -> Result<HttpResponse> {
    let mut ctx = Context::new();
    ctx.insert("cv", cv_data.get_ref());

    let html = tera
        .render("pages/cv.html", &ctx)
        .map_err(|e| {
            eprintln!("❌  Erreur rendu template : {}", e);
            actix_web::error::ErrorInternalServerError("Template error")
        })?;

    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html))
}

// Handler pour le CV anglais — /en
pub async fn render_en(
    tera:    web::Data<Tera>,
    cv_data: web::Data<crate::data::cv::CvData>,
) -> Result<HttpResponse> {
    // Charge les données anglaises au lieu des françaises
    let en_data = crate::data::cv_en::load();

    let mut ctx = Context::new();
    ctx.insert("cv", &en_data);

    let html = tera
        .render("pages/cv.html", &ctx)
        .map_err(|e| {
            eprintln!("❌  Erreur rendu template EN : {}", e);
            actix_web::error::ErrorInternalServerError("Template error")
        })?;

    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html))
}
