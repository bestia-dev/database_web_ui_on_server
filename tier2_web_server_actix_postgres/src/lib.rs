// database_web_ui_on_server/tier2_web_server_actix_postgres/src/lib.rs

mod actix_mod;
mod deadpool_mod;
mod postgres_mod;

pub use actix_mod::*;
pub use deadpool_mod::deadpool_postgres_start;

use thiserror::Error;
#[derive(Error, Debug)]
pub enum LibraryError {
    #[error("Unknown error.")]
    Unknown,
}
// ANSI colors for Linux terminal
// https://github.com/shiena/ansicolor/blob/master/README.md
#[allow(dead_code)]
pub const RED: &str = "\x1b[31m";
#[allow(dead_code)]
pub const YELLOW: &str = "\x1b[33m";
#[allow(dead_code)]
pub const GREEN: &str = "\x1b[32m";
#[allow(dead_code)]
pub const RESET: &str = "\x1b[0m";
