use axum::{async_trait, extract::FromRequestParts, http::request::Parts};
use uuid::Uuid;

use crate::error::Error;

#[derive(Clone, Debug)]
pub struct Context {
    id: Uuid,
    tenant: Uuid,
    name: String,
    email: String
}

impl Context {
    pub fn new(id: Uuid, tenant: Uuid, name: String, email: String) -> Self {
        Self { id, tenant, name, email }
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

    pub fn email(&self) -> &str {
        &self.email.as_str()
    }
}

#[async_trait]
impl<S: Send + Sync> FromRequestParts<S> for Context {
	type Rejection = Error;

	async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Error> {
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