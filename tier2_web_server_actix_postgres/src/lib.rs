// database_web_ui_on_server/tier2_web_server_actix_postgres/src/lib.rs

mod actix_mod;
mod deadpool_mod;
mod error_mod;
mod postgres_mod;
mod webpage_hits_mod;

pub use actix_mod::config_route_main;
pub use deadpool_mod::deadpool_start_and_check;
