// src/pdf.rs
use actix_web::{web, HttpResponse, Result};
use chromiumoxide::browser::{Browser, BrowserConfig};
use futures::StreamExt;

pub async fn export(
    tera:    web::Data<tera::Tera>,
    cv_data: web::Data<crate::data::cv::CvData>,
) -> Result<HttpResponse> {

    // 1. Génère le HTML du CV
    let mut ctx = tera::Context::new();
    ctx.insert("cv", cv_data.get_ref());

    let html = tera
        .render("pages/cv.html", &ctx)
        .map_err(|e| {
            eprintln!("❌  Erreur rendu template : {}", e);
            actix_web::error::ErrorInternalServerError("Template error")
        })?;

    // 2. Lance Chromium headless
    let config = BrowserConfig::builder()
        .chrome_executable("/usr/bin/chromium-browser")
        .no_sandbox()
        .build()
        .map_err(|e| {
            eprintln!("❌  Erreur config Chromium : {}", e);
            actix_web::error::ErrorInternalServerError("Browser config error")
        })?;

    let (mut browser, mut handler) = Browser::launch(config)
        .await
        .map_err(|e| {
            eprintln!("❌  Erreur lancement Chromium : {}", e);
            actix_web::error::ErrorInternalServerError("Browser launch error")
        })?;

    // 3. Lance le handler en arrière-plan
    let handler_task = tokio::spawn(async move {
        while let Some(_) = handler.next().await {}
    });

    // 4. Ouvre un onglet
    let page = browser
        .new_page("about:blank")
        .await
        .map_err(|e| {
            eprintln!("❌  Erreur nouvelle page : {}", e);
            actix_web::error::ErrorInternalServerError("Page error")
        })?;

    // 5. Injecte le HTML encodé en base64
    let encoded = base64_encode(&html);
    page.goto(format!("data:text/html;base64,{}", encoded))
        .await
        .map_err(|e| {
            eprintln!("❌  Erreur navigation : {}", e);
            actix_web::error::ErrorInternalServerError("Navigation error")
        })?;

    // 6. Attend le chargement complet
    page.wait_for_navigation()
        .await
        .map_err(|e| {
            eprintln!("❌  Erreur attente navigation : {}", e);
            actix_web::error::ErrorInternalServerError("Navigation wait error")
        })?;

    // 7. Génère le PDF en A4 avec les couleurs de fond
    let pdf_bytes = page
        .pdf(chromiumoxide::cdp::browser_protocol::page::PrintToPdfParams {
            print_background: Some(true),
            ..Default::default()
        })
        .await
        .map_err(|e| {
            eprintln!("❌  Erreur génération PDF : {}", e);
            actix_web::error::ErrorInternalServerError("PDF generation error")
        })?;

    // 8. Ferme le navigateur
    browser.close().await.ok();
    handler_task.abort();

    // 9. Renvoie le PDF comme téléchargement
    Ok(HttpResponse::Ok()
        .content_type("application/pdf")
        .append_header((
            "Content-Disposition",
            "attachment; filename=\"nicolas-garric-cv.pdf\"",
        ))
        .body(pdf_bytes))
}

// Encodage base64 sans dépendance externe
fn base64_encode(input: &str) -> String {
    const CHARS: &[u8] =
        b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let bytes = input.as_bytes();
    let mut result = String::with_capacity((bytes.len() + 2) / 3 * 4);

    for chunk in bytes.chunks(3) {
        let b0 = chunk[0] as usize;
        let b1 = if chunk.len() > 1 { chunk[1] as usize } else { 0 };
        let b2 = if chunk.len() > 2 { chunk[2] as usize } else { 0 };

        result.push(CHARS[b0 >> 2] as char);
        result.push(CHARS[((b0 & 3) << 4) | (b1 >> 4)] as char);
        result.push(if chunk.len() > 1 {
            CHARS[((b1 & 15) << 2) | (b2 >> 6)] as char
        } else {
            '='
        });
        result.push(if chunk.len() > 2 {
            CHARS[b2 & 63] as char
        } else {
            '='
        });
    }
    result
}

// Export PDF version japonaise
pub async fn export_ja(
    tera:    web::Data<tera::Tera>,
    cv_data: web::Data<crate::data::cv::CvData>,
) -> Result<HttpResponse> {
    let ja_data = crate::data::cv_ja::load();

    let mut ctx = tera::Context::new();
    ctx.insert("cv", &ja_data);

    let html = tera
        .render("pages/cv.html", &ctx)
        .map_err(|e| {
            eprintln!("❌  Erreur rendu template JA : {}", e);
            actix_web::error::ErrorInternalServerError("Template error")
        })?;

    let config = BrowserConfig::builder()
        .chrome_executable("/usr/bin/chromium-browser")
        .no_sandbox()
        .build()
        .map_err(|_| actix_web::error::ErrorInternalServerError("Browser config error"))?;

    let (mut browser, mut handler) = Browser::launch(config)
        .await
        .map_err(|_| actix_web::error::ErrorInternalServerError("Browser launch error"))?;

    let handler_task = tokio::spawn(async move {
        while let Some(_) = handler.next().await {}
    });

    let page = browser
        .new_page("about:blank")
        .await
        .map_err(|_| actix_web::error::ErrorInternalServerError("Page error"))?;

    let encoded = base64_encode(&html);
    page.goto(format!("data:text/html;base64,{}", encoded))
        .await
        .map_err(|_| actix_web::error::ErrorInternalServerError("Navigation error"))?;

    page.wait_for_navigation()
        .await
        .map_err(|_| actix_web::error::ErrorInternalServerError("Navigation wait error"))?;

    let pdf_bytes = page
        .pdf(chromiumoxide::cdp::browser_protocol::page::PrintToPdfParams {
            print_background: Some(true),
            ..Default::default()
        })
        .await
        .map_err(|_| actix_web::error::ErrorInternalServerError("PDF generation error"))?;

    browser.close().await.ok();
    handler_task.abort();

    Ok(HttpResponse::Ok()
        .content_type("application/pdf")
        .append_header((
            "Content-Disposition",
            "attachment; filename=\"nicolas-garric-cv-ja.pdf\"",
        ))
        .body(pdf_bytes))
}

// Export PDF version anglaise
pub async fn export_en(
    tera:    web::Data<tera::Tera>,
    cv_data: web::Data<crate::data::cv::CvData>,
) -> Result<HttpResponse> {
    let en_data = crate::data::cv_en::load();

    let mut ctx = tera::Context::new();
    ctx.insert("cv", &en_data);

    let html = tera
        .render("pages/cv.html", &ctx)
        .map_err(|e| {
            eprintln!("❌  Erreur rendu template EN : {}", e);
            actix_web::error::ErrorInternalServerError("Template error")
        })?;

    // Même logique que export() — Chromium génère le PDF
    let config = BrowserConfig::builder()
        .chrome_executable("/usr/bin/chromium-browser")
        .no_sandbox()
        .build()
        .map_err(|_| actix_web::error::ErrorInternalServerError("Browser config error"))?;

    let (mut browser, mut handler) = Browser::launch(config)
        .await
        .map_err(|_| actix_web::error::ErrorInternalServerError("Browser launch error"))?;

    let handler_task = tokio::spawn(async move {
        while let Some(_) = handler.next().await {}
    });

    let page = browser
        .new_page("about:blank")
        .await
        .map_err(|_| actix_web::error::ErrorInternalServerError("Page error"))?;

    let encoded = base64_encode(&html);
    page.goto(format!("data:text/html;base64,{}", encoded))
        .await
        .map_err(|_| actix_web::error::ErrorInternalServerError("Navigation error"))?;

    page.wait_for_navigation()
        .await
        .map_err(|_| actix_web::error::ErrorInternalServerError("Navigation wait error"))?;

    let pdf_bytes = page
        .pdf(chromiumoxide::cdp::browser_protocol::page::PrintToPdfParams {
            print_background: Some(true),
            ..Default::default()
        })
        .await
        .map_err(|_| actix_web::error::ErrorInternalServerError("PDF generation error"))?;

    browser.close().await.ok();
    handler_task.abort();

    Ok(HttpResponse::Ok()
        .content_type("application/pdf")
        .append_header((
            "Content-Disposition",
            "attachment; filename=\"nicolas-garric-cv-en.pdf\"",
        ))
        .body(pdf_bytes))
}
