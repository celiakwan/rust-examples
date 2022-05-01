use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use diesel::result::Error as DieselError;
use serde::Deserialize;
use std::fmt;

#[derive(Deserialize, Debug)]
pub struct ServiceError {
    pub status_code: u16,
    pub message: String,
}

impl ServiceError {
    pub fn new(status_code: u16, message: String) -> ServiceError {
        ServiceError {
            status_code,
            message
        }
    }
}

impl fmt::Display for ServiceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.message.as_str())
    }
}

impl From<DieselError> for ServiceError {
    fn from(error: DieselError) -> ServiceError {
        match error {
            DieselError::DatabaseError(_, e) => ServiceError::new(409, e.message().to_string()),
            DieselError::NotFound => ServiceError::new(404, "Record not found".to_string()),
            err => ServiceError::new(500, format!("Diesel error: {}", err))
        }
    }
}

impl ResponseError for ServiceError {
    fn error_response(&self) -> HttpResponse {
        let status_code = match StatusCode::from_u16(self.status_code) {
            Ok(status_code) => status_code,
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR
        };
        let message = match status_code.as_u16() < 500 {
            true => self.message.clone(),
            false => {
                error!("{}", self.message);
                "Internal server error".to_string()
            }
        };
        HttpResponse::build(status_code).json(message)
    }
}
