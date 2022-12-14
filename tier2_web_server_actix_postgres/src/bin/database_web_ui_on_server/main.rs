// database_web_ui_on_server/tier2_web_server_actix_postgres/src/bin/database_web_ui_on_server/main.rs
use tier2_web_server_actix_postgres as tier2;

/// the binary executable entry point
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    pretty_env_logger::init();

    println!("Actix web server started on localhost:8080!");
    println!("Test it with curl or browser:");
    println!("http://localhost:8080/database_web_ui_on_server/webpage_hits/webpage_hits_list");

    // connection pool for postgres to reuse connections for better performance
    let db_pool = tier2::deadpool_start_and_check().await;

    let http_server_result = actix_web::HttpServer::new(move || {
        actix_web::App::new()
            // app_data is cloned for every worker thread
            .app_data(actix_web::web::Data::new(db_pool.clone()))
            // the route is configured near the implementation code
            .configure(tier2::config_route_main)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await;

    println!("");
    println!("Actix web server stopped!");
    // return
    http_server_result
}
