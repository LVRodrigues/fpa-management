use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use chrono::{DateTime, Utc};
use log::{debug, error};
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
    MultipleRowsAffected,
    KeyNotFound,
    JWKSNotFound,
    TokenInvalid,
    ContextInvalid,
    DatabaseConnection,
    DatabaseTransaction,
    RegisterUser,
    ProjectCreate,
    ProjectNameDuplicated,
    ProjectFactorCreate,
    ProjectEmpiricalCreate,
    ProductivityInvalid,
    ProjectConstraints,
    ProjectUpdate,
    EmpiricalInvalid,
    FrontierCreate,
    FrontierNameDuplicated,
    FrontierUpdate,
    FrontierConstraints,
    NotFunctionData,
    NotFunctionTransaction,
    FunctionCreate,
    FunctionTypeUpdateError,
    FunctionNameDuplicated,
    FunctionUpdate,
    FunctionConstraints,
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
        debug!("{}", self);

        let (code, message) = match self {
            Error::TokenInvalid
            | Error::ContextInvalid
            | Error::KeyNotFound
            | Error::Unauthorized => (
                StatusCode::UNAUTHORIZED,
                ErrorResponse {
                    id: Uuid::now_v7(),
                    time: Utc::now(),
                    error: "AUTHENTICATION",
                    message: "Authentication error. Request a new Access Token.",
                },
            ),
            // Error::Forbidden => {
            //     (
            //         StatusCode::FORBIDDEN,
            //             ErrorResponse {
            //             id: Uuid::now_v7(),
            //             time: Utc::now(),
            //             error: "AUTHORIZATION",
            //             message: "Não autorizado para esta operação."
            //         }
            //     )
            // }
            Error::NotFound => (
                StatusCode::NOT_FOUND,
                ErrorResponse {
                    id: Uuid::now_v7(),
                    time: Utc::now(),
                    error: "NOT_FOUND",
                    message: "Resource not found with the specified parameters.",
                },
            ),
            Error::MultipleRowsAffected => (
                StatusCode::CONFLICT,
                ErrorResponse {
                    id: Uuid::now_v7(),
                    time: Utc::now(),
                    error: "DATABASE_ERROR",
                    message: "Database in inconsistent state.",
                },
            ),
            Error::ProjectConstraints | Error::FrontierConstraints | Error::FunctionConstraints => {
                (
                    StatusCode::PRECONDITION_FAILED,
                    ErrorResponse {
                        id: Uuid::now_v7(),
                        time: Utc::now(),
                        error: "CONSTRAINT_ERROR",
                        message: "Registry has related data.",
                    },
                )
            }
            Error::JWKSNotFound | Error::DatabaseConnection | Error::DatabaseTransaction => (
                StatusCode::SERVICE_UNAVAILABLE,
                ErrorResponse {
                    id: Uuid::now_v7(),
                    time: Utc::now(),
                    error: "SERVICE_ERROR",
                    message: "Service temporarily unavailable.",
                },
            ),
            Error::ProductivityInvalid => (
                StatusCode::NOT_ACCEPTABLE,
                ErrorResponse {
                    id: Uuid::now_v7(),
                    time: Utc::now(),
                    error: "NOT_ACCEPTABLE",
                    message: "Productivity must have a value between 1 and 50.",
                },
            ),
            Error::EmpiricalInvalid => (
                StatusCode::NOT_ACCEPTABLE,
                ErrorResponse {
                    id: Uuid::now_v7(),
                    time: Utc::now(),
                    error: "NOT_ACCEPTABLE",
                    message: "Empirical adjustment factors must have a value between 0 and 100.",
                },
            ),
            Error::ProjectNameDuplicated
            | Error::FrontierNameDuplicated
            | Error::FunctionNameDuplicated => (
                StatusCode::CONFLICT,
                ErrorResponse {
                    id: Uuid::now_v7(),
                    time: Utc::now(),
                    error: "NAME_DUPLICATED",
                    message: "The name must be unique for this scope.",
                },
            ),
            Error::NotFunctionData
            | Error::NotFunctionTransaction
            | Error::FunctionTypeUpdateError => (
                StatusCode::NOT_ACCEPTABLE,
                ErrorResponse {
                    id: Uuid::now_v7(),
                    time: Utc::now(),
                    error: "NOT_ACCEPTABLE",
                    message: "The Function Type is not valid for this scope.",
                },
            ),
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ErrorResponse {
                    id: Uuid::now_v7(),
                    time: Utc::now(),
                    error: "SERVICE_ERROR",
                    message: "Internal service error.",
                },
            ),
        };

        error!("{}", json!(message));
        (code, Json(json!(message))).into_response()
    }
}

impl From<reqwest::Error> for Error {
    fn from(value: reqwest::Error) -> Self {
        error!("{}", value);
        Error::JWKSNotFound
    }
}

impl From<jsonwebtoken::errors::Error> for Error {
    fn from(value: jsonwebtoken::errors::Error) -> Self {
        error!("{}", value);
        Error::TokenInvalid
    }
}

impl From<sea_orm::DbErr> for Error {
    fn from(value: sea_orm::DbErr) -> Self {
        error!("{}", value);
        Error::DatabaseTransaction
    }
}
