use std::fmt::{Debug, Formatter, Result};

use actix_web::http::{header, StatusCode};
use actix_web::{HttpResponse, ResponseError};

use crate::errors::error_chain_fmt;

#[derive(thiserror::Error)]
pub enum SaleError {
    #[error("{0}")]
    ValidationError(String),
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

impl Debug for SaleError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        error_chain_fmt(self, f)
    }
}

impl ResponseError for SaleError {
    fn status_code(&self) -> StatusCode {
        match self {
            SaleError::ValidationError(_) => StatusCode::BAD_REQUEST,
            SaleError::UnexpectedError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header((header::CONTENT_TYPE, "application/json"))
            .json(serde_json::json!({
                "error": self.to_string(),
            }))
    }
}
