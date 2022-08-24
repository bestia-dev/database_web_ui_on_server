// database_web_ui_on_server/tier2_web_server_actix_postgres/src/actix_mod.rs
pub async fn hit_counter_list() -> impl actix_web::Responder {
    println!("database_web_ui_on_server/hit_counter_list");

    actix_web::HttpResponse::Ok()
        .append_header(actix_web::http::header::ContentType(mime::TEXT_HTML_UTF_8))
        .append_header(actix_web::http::header::CacheControl(vec![
            actix_web::http::header::CacheDirective::NoStore,
        ]))
        .body(format!("Hello world!"))
}
