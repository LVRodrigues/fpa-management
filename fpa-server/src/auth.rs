use crate::{error::{Error, Result}, jwks, context::Context};

use axum::{http::{Request, header}, middleware::Next, response::Response, body::Body};
use jsonwebtoken::{decode_header, Validation, decode, DecodingKey};
use serde::{Serialize, Deserialize};

const BEARER: &str = "Bearer ";
const AUDIENCE: &str = "account";

/**
 * Claims is used to extract information from the Token.
 */
#[derive(Debug, Serialize, Deserialize)]
struct Claims {
   sub: String,
   name: String
}

pub async fn require(mut request: Request<Body>, next: Next) -> Result<Response> {
    let token = request.headers()
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

    let claims  = decode::<Claims>(&token, &key, &validation)?.claims;
    let context = Context::new(claims.sub, claims.name);

    request.extensions_mut().insert(context);
    
    Ok(next.run(request).await)
}