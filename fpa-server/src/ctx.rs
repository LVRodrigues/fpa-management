use axum::{async_trait, extract::FromRequestParts, http::request::Parts};
use uuid::Uuid;

use crate::error::{Error, Result};

#[derive(Clone, Debug)]
pub struct Context {
    id: Uuid,
    tenant: Uuid,
    name: String,
}

impl Context {
    pub fn new(id: Uuid, tenant: Uuid, name: String) -> Self {
        Self { id, tenant, name }
    }

    pub fn id(&self) -> &Uuid {
        &self.id
    }

    pub fn tenant(&self) -> &Uuid {
        &self.tenant
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

		let context = parts
			.extensions
			.get::<Context>()
            .ok_or(Error::ContextInvalid)
            .unwrap()
            .clone();

        Ok(context)
	}
}