
use axum::{http::StatusCode, response::{IntoResponse, Response}, Json};
use chrono::{DateTime, Utc};
use serde::Serialize;
use serde_json::json;
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, strum_macros::AsRefStr)]
pub enum Error {
    Unauthorized,
    Forbidden,
    ParamInvalid,
    NotFound,
    KeyNotFound,
    JWKSNotFound,
    TokenInvalid,
    ContextInvalid,
    DatabaseConnection,
}

impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}

impl IntoResponse for Error {
	fn into_response(self) -> Response {
		println!("==> {:<12} - {self:?}", "ERROR");

        #[derive(Serialize)]
        struct ErrorResponse<'a> {
            id: Uuid,
            time: DateTime<Utc>,
            error: &'a str,
            message: &'a str,
        }

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
                        message: "Erro de autenticação."
                    }
                )
            }
            Error::Forbidden => {
                (
                    StatusCode::FORBIDDEN,
                        ErrorResponse {
                        id: Uuid::new_v4(),
                        time: Utc::now(),
                        error: "AUTHORIZATION",
                        message: "Não autorizado para esta operação."
                    }
                )
            }
            Error::NotFound => {
                (
                    StatusCode::NOT_FOUND,
                    ErrorResponse {
                        id: Uuid::new_v4(),
                        time: Utc::now(),
                        error: "PARAM_INVALID",
                        message: "Recurso não localizado com os parâmetros informados."
                    }
                )
            }
            Error::JWKSNotFound |
            Error::DatabaseConnection => {
                (
                    StatusCode::SERVICE_UNAVAILABLE,
                    ErrorResponse {
                        id: Uuid::new_v4(),
                        time: Utc::now(),
                        error: "SERVICE_ERROR",
                        message: "Serviço temporariamente indisponível."
                    }
                )
            }
            _ => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                        ErrorResponse {
                        id: Uuid::new_v4(),
                        time: Utc::now(),
                        error: "SERVICE_ERROR",
                        message: "Erro interno do serviço."
                    }
                )
        };

        println!("--->>> error: \n{}", json!(message));
        println!();
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
