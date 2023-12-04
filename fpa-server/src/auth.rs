use crate::{error::{Error, Result}, jwks};

use axum::{http::{Request, header}, middleware::Next, response::Response};
use jsonwebtoken::{decode_header, Validation, decode, DecodingKey};
use serde::{Serialize, Deserialize};

const BEARER: &str = "Bearer ";

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
   sub: String,
   name: String
}

pub async fn require<B>(request: Request<B>, next: Next<B>) -> Result<Response> {
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
    validation.set_audience(&["account"]);

    let claims  = decode::<Claims>(&token, &key, &validation)?.claims;
    println!("{:?}", claims);
    
    Ok(next.run(request).await)
}