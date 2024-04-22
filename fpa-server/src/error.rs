
use axum::{http::StatusCode, response::{IntoResponse, Response}, Json};
use chrono::{DateTime, Utc};
use serde::Serialize;
use serde_json::json;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, strum_macros::AsRefStr)]
pub enum Error {
    Unauthorized,
    // Forbidden,
    // ParamInvalid,
    NotFound,
    KeyNotFound,
    JWKSNotFound,
    TokenInvalid,
    ContextInvalid,
    DatabaseConnection,
    DatabaseTransaction,
    RegisterUser,
    ProjectCreate,
}

impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}

/// Information about the error that occurred.
#[derive(Serialize, ToSchema)]
#[schema(as=Error)]
#[serde(rename = "Error")] 
pub struct ErrorResponse<'a> {
    /// Error Unique Identifier
    id: Uuid,
    /// Error time.
    time: DateTime<Utc>,
    /// Error title.
    error: &'a str,
    /// Error message.
    message: &'a str,
}

impl IntoResponse for Error {
	fn into_response(self) -> Response {
		println!("==> {:<12} - {self:?}", "ERROR");

        let (code, message) = match self {
            Error::TokenInvalid |
            Error::ContextInvalid |
            Error::KeyNotFound |
            Error::Unauthorized => {
                (   
                    StatusCode::UNAUTHORIZED, 
                    ErrorResponse {
                        id: Uuid::new_v4(),
                        time: Utc::now(),
                        error: "AUTHENTICATION",
                        message: "Authentication error. Request a new Access Token."
                    }
                )
            }
            // Error::Forbidden => {
            //     (
            //         StatusCode::FORBIDDEN,
            //             ErrorResponse {
            //             id: Uuid::new_v4(),
            //             time: Utc::now(),
            //             error: "AUTHORIZATION",
            //             message: "Não autorizado para esta operação."
            //         }
            //     )
            // }
            Error::NotFound => {
                (
                    StatusCode::NOT_FOUND,
                    ErrorResponse {
                        id: Uuid::new_v4(),
                        time: Utc::now(),
                        error: "NOT_FOUND",
                        message: "Resource not found with the specified parameters."
                    }
                )
            }
            Error::JWKSNotFound |
            Error::DatabaseConnection | 
            Error::DatabaseTransaction => {
                (
                    StatusCode::SERVICE_UNAVAILABLE,
                    ErrorResponse {
                        id: Uuid::new_v4(),
                        time: Utc::now(),
                        error: "SERVICE_ERROR",
                        message: "Service temporarily unavailable."
                    }
                )
            }
            _ => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                        ErrorResponse {
                        id: Uuid::new_v4(),
                        time: Utc::now(),
                        error: "SERVICE_ERROR",
                        message: "Internal service error."
                    }
                )
        };

        println!("--->>> error: {}", json!(message));
        (code, Json(json!(message))).into_response()
	}
}

impl From<reqwest::Error> for Error {
    fn from(value: reqwest::Error) -> Self {
        println!("==> {:<12} - {value:?}", "ERROR");
        Error::JWKSNotFound
    }
}

impl From<jsonwebtoken::errors::Error> for Error {
    fn from(value: jsonwebtoken::errors::Error) -> Self {
        println!("==> {:<12} - {value:?}", "ERROR");
        Error::TokenInvalid
    }
}

impl From<sea_orm::DbErr> for Error {
    fn from(value: sea_orm::DbErr) -> Self {
        println!("==> {:<12} - {value:?}", "ERROR");
        Error::DatabaseTransaction
    }
}