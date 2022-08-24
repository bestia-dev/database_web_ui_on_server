// database_web_ui_on_server/tier2_web_server_actix_postgres/src/bin/database_web_ui_on_server/main.rs
use tier2_web_server_actix_postgres::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Actix web server started on localhost:8080!");
    println!("Test it with curl or browser:");
    println!("http://localhost:8080/database_web_ui_on_server/hit_counter_list");

    println!("Start server");
    let http_server_result = actix_web::HttpServer::new(move || {
        actix_web::App::new().route(
            "/database_web_ui_on_server/hit_counter_list",
            actix_web::web::get().to(hit_counter_list),
        )
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await;

    println!("");
    println!("Actix web server stopped!");
    // return
    http_server_result
}
