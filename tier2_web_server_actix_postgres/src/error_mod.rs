// error_mod.rs

// thiserror makes the Display trait for variants
// first string is for user message, second string is for developer log
#[derive(thiserror::Error, Debug)]
pub enum AppError {
    #[error("Database connection error.")]
    DatabaseConnection,

    #[error("Query error: {user_friendly}")]
    QueryError {
        source_error: tokio_postgres::Error,
        user_friendly: String,
        developer_friendly: String,
        source_line_column: String,
    },

    #[error("The value does not exist in web query: {user_friendly}")]
    GetValueFromQuery {
        user_friendly: String,
        developer_friendly: String,
        source_line_column: String,
    },

    #[error("The value is not i32: {user_friendly}")]
    GetI32FromQuery {
        user_friendly: String,
        developer_friendly: String,
        source_line_column: String,
    },

    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

impl actix_web::ResponseError for AppError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match *self {
            _ => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> actix_web::HttpResponse {
        let status_code = self.status_code();
        // more information for the developer
        // I need the exact time to match the user message with the log
        let time = time_epoch();
        // log is developer friendly with many more info
        log::error!("{time} {}\n{:#?}", self, self);
        // only the user-friendly error for the user
        actix_web::HttpResponse::build(status_code).body(format!("{time} {}", self))
    }
}

pub fn time_epoch() -> u128 {
    let time = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis();
    time
}
