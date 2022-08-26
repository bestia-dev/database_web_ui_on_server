// database_web_ui_on_server/tier2_web_server_actix_postgres/src/actix_mod.rs
pub async fn hit_counter_list(
    db_pool: actix_web::web::Data<deadpool_postgres::Pool>,
) -> impl actix_web::Responder {
    println!("database_web_ui_on_server/hit_counter_list");

    let body = match crate::postgres_mod::db_hit_counter_list(db_pool).await {
        Err(_err) => "Error reading database tables.".to_string(),
        Ok(vec_row) => {
            let mut body = String::new();
            for r in vec_row {
                let webpage: String = r.get(0);
                let counter: i32 = r.get(1);
                body.push_str(&format!(r##"<p>{webpage}  {counter}</p>"##));
            }
            body
        }
    };

    actix_web::HttpResponse::Ok()
        .append_header(actix_web::http::header::ContentType(mime::TEXT_HTML_UTF_8))
        .append_header(actix_web::http::header::CacheControl(vec![
            actix_web::http::header::CacheDirective::NoStore,
        ]))
        .body(body)
}
