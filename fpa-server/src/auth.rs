use std::sync::Arc;

use crate::{
    ctx::Context, error::Error, jwks, state::AppState
};

use axum::{
    body::Body, extract::State, http::{header, Request}, middleware::Next, response::Response
};
use jsonwebtoken::{decode, decode_header, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

const BEARER: &str = "Bearer ";
const AUDIENCE: &str = "account";

/**
 * Claims is used to extract information from the Token.
 */
#[derive(Clone, Debug, Serialize, Deserialize)]
struct Claims {
    sub: Uuid,
    tenant: Uuid,
    name: String,
}

impl Claims {
    fn to_context(&self) -> Context {
        Context::new(self.sub, self.tenant, self.name.to_owned())
    }
}

pub async fn require(State(state): State<Arc<AppState>>, mut request: Request<Body>, next: Next) -> Result<Response, Error> {
    println!("==> {:<12} - require", "AUTH");

    if !jwks::is_prepared() {
        let config = state.configuration();
        jwks::prepare(config).await?;
    }

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
    request.extensions_mut().insert(claims.to_context());

    Ok(next.run(request).await)
}
