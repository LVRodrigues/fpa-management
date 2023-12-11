use axum::{async_trait, extract::FromRequestParts, http::request::Parts};

use crate::error::{Error, Result};

#[derive(Clone, Debug)]
pub struct Context {
    id: String,
    name: String,
}

impl Context {
    pub fn new(id: String, name: String) -> Self {
        Self { id, name }
    }

    pub fn id(&self) -> &str {
        &self.id.as_str()
    }

    pub fn name(&self) -> &str {
        &self.name.as_str()
    }
}

#[async_trait]
impl<S: Send + Sync> FromRequestParts<S> for Context {
	type Rejection = Error;

	async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self> {
		println!("==> {:<12} - Context", "EXTRACTOR");

		let claims = parts
			.extensions
			.get::<Context>()
            .unwrap()
            .clone();

        Ok(claims)
	}
}