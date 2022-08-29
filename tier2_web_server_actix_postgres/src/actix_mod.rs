// database_web_ui_on_server/tier2_web_server_actix_postgres/src/actix_mod.rs

/// configure the route with scope
/// so the routing code is near to the implementation code
pub fn config_route_main(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(actix_files::Files::new(
        "/database_web_ui_on_server/css",
        "./database_web_ui_on_server/css/",
    ))
    .service(
        actix_web::web::scope("/database_web_ui_on_server/webpage_hits")
            .configure(crate::webpage_hits_mod::config_route_webpage_hits),
    );
}

/// fn to return a response when we have the body
pub fn return_response_no_cache(body: String) -> actix_web::HttpResponse {
    actix_web::HttpResponse::Ok()
        .append_header(actix_web::http::header::ContentType(mime::TEXT_HTML_UTF_8))
        .append_header(actix_web::http::header::CacheControl(vec![
            actix_web::http::header::CacheDirective::NoStore,
        ]))
        .body(body)
}

// data from GET query
// [("id", "496953237"), ("webpage", "webpage short url"), ("hit_count", "0")]
pub fn get_value_from_query<'a>(
    query: &'a actix_web::web::Query<Vec<(String, String)>>,
    name: &str,
) -> Option<&'a str> {
    for x in query.0.iter() {
        if x.0 == name {
            return Some(&x.1);
        }
    }
    None
}
