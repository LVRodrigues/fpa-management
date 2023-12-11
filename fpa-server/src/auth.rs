use crate::{
    error::{Error, Result},
    jwks,
};

use axum::{
    body::Body,
    http::{header, Request, request::Parts},
    middleware::Next,
    response::Response, async_trait, extract::FromRequestParts,
};
use jsonwebtoken::{decode, decode_header, DecodingKey, Validation};
use serde::{Deserialize, Serialize};

const BEARER: &str = "Bearer ";
const AUDIENCE: &str = "account";

/**
 * Claims is used to extract information from the Token.
 */
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub name: String,
}

pub async fn require(mut request: Request<Body>, next: Next) -> Result<Response> {
    println!("==> {:<12} - require", "AUTH");

    let token = request
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok())
        .and_then(|value| {
            if value.starts_with(BEARER) {
                Some(value[BEARER.len()..].to_owned())
            } else {
                None
            }
        });
    let token = match token {
        Some(v) => v,
        None => return Err(Error::Unauthorized),
    };

    let header = decode_header(&token)?;

    let key = jwks::key(header.kid.unwrap().clone())?;
    let key = DecodingKey::from_jwk(&key.to_jwk()).unwrap();

    let mut validation = Validation::new(header.alg);
    validation.set_audience(&[AUDIENCE]);

    let claims = decode::<Claims>(&token, &key, &validation)?.claims;
    request.extensions_mut().insert(claims);

    Ok(next.run(request).await)
}

#[async_trait]
impl<S: Send + Sync> FromRequestParts<S> for Claims {
	type Rejection = Error;

	async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self> {
		println!("->> {:<12} - Claims", "EXTRACTOR");

		let claims = parts
			.extensions
			.get::<Claims>()
            .unwrap()
            .clone();

        Ok(claims)
	}
}