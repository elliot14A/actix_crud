use actix_web::{ResponseError, HttpResponse, http::StatusCode};
use derive_more::Display;


#[derive(Debug, Display)]
pub enum ServiceError {
    #[display(fmt="BadRequest")]
    DBQueryError(String),
    #[display(fmt="")]
    InvalidEmailOrPassword,
    Forbidden,
}

impl ResponseError for ServiceError {
    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        match self {
            Self::DBQueryError(ref message) => HttpResponse::BadRequest().json(message),
            Self::InvalidEmailOrPassword => HttpResponse::Unauthorized().finish(),
            Self::Forbidden => HttpResponse::Forbidden().finish()
        }
    }

    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            Self::DBQueryError(_) => StatusCode::BAD_REQUEST,
            Self::InvalidEmailOrPassword => StatusCode::UNAUTHORIZED,
            Self::Forbidden => StatusCode::FORBIDDEN
        }
    }
}
