use axum::{http::StatusCode, response::{IntoResponse, Response}};
use serde::Serialize;

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

impl std::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}

impl IntoResponse for Error {
	fn into_response(self) -> Response {
		println!("==> {:<12} - {self:?}", "INTO_RES");

		// Create a placeholder Axum reponse.
		let mut response = StatusCode::INTERNAL_SERVER_ERROR.into_response();

		// Insert the Error into the reponse.
		response.extensions_mut().insert(self);

		response
	}
}

impl From<reqwest::Error> for Error {
    fn from(value: reqwest::Error) -> Self {
        println!("==> {:<12} - {value:?}", "ERROR ");
        Error::JWKSNotFound
    }
}

impl From<jsonwebtoken::errors::Error> for Error {
    fn from(value: jsonwebtoken::errors::Error) -> Self {
        println!("==> {:<12} - {value:?}", "ERROR ");
        Error::TokenInvalid
    }
}

impl Error {
    pub fn client_status_and_error(&self) -> (StatusCode, ClientError) {
        match self {
            Error::TokenInvalid |
            Error::ContextInvalid |
            Error::KeyNotFound |
            Error::Unauthorized => {
                (StatusCode::UNAUTHORIZED, ClientError::AUTHENTICATION)
            }
            Error::Forbidden => {
                (StatusCode::FORBIDDEN, ClientError::AUTHORIZATION)
            }
            Error::NotFound => {
                (StatusCode::NOT_FOUND, ClientError::PARAMS_INVALID)
            }
            Error::JWKSNotFound |
            Error::DatabaseConnection => {
                (StatusCode::SERVICE_UNAVAILABLE, ClientError::SERVICE_ERROR)
            }
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ClientError::SERVICE_ERROR
            )
        }
    }
}

#[derive(Debug, strum_macros::AsRefStr)]
#[allow(non_camel_case_types)]
pub enum ClientError {
    AUTHENTICATION,
    AUTHORIZATION,
    PARAMS_INVALID,
    SERVICE_ERROR
}

