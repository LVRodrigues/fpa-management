
use std::collections::HashMap;

use serde_json::{json, Value};
use utoipa::{openapi::{security::{Flow, OAuth2, Password, Scopes, SecurityScheme}, Components}, Modify, OpenApi};


#[derive(OpenApi)]
#[openapi(
    tags(
        [name = "Status", description = "Check service health."],
        [name = "Projects", description = "FPA Projects management."]
    ),
    paths(
        crate::handlers::health,
        crate::handlers::projects::list,
        crate::handlers::projects::by_id,
        crate::handlers::projects::create,
        crate::handlers::projects::update,
        crate::handlers::projects::remove,
    ),
    components(
        schemas(
            crate::model::sea_orm_active_enums::Empirical,
            crate::model::sea_orm_active_enums::Factor,
            crate::model::sea_orm_active_enums::Influence,
            crate::model::sea_orm_active_enums::FunctionType,
            crate::model::users::Model,
            crate::model::page::Users,
            crate::model::projects::Model,
            crate::model::page::Projects,
            crate::model::empiricals::Model,
            crate::model::page::Empiricals,
            crate::model::factors::Model,
            crate::model::page::Factors,
            crate::model::versions::Model,
            crate::model::page::Versions,
            crate::error::ErrorResponse,
            crate::handlers::projects::ProjectCreateParam,
            crate::handlers::projects::ProjectUpdateParam,
        ),
    ),
    modifiers(&SecuritySchemas, &InfoModifier),
)]
pub struct ApiDoc;    

struct InfoModifier;
impl Modify for InfoModifier {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let mut logo: HashMap<String, Value> = HashMap::new();
        let value = json!({
            //"url": "https://github.com/LVRodrigues/apf-calc/blob/main/src/assets/logo.png?raw=true", 
            "url": "/assets/logo.png", 
            "backgroundColor": "#FFFFFF"
        });
        logo.insert("x-logo".to_owned(), value);
        openapi.info.extensions = Some(logo);

        let desc = r#"
# Introduction

Project Management using Function Points Analysis.

# Cross-Origin Resource Sharing

This API features Cross-Origin Resource Sharing (CORS) implemented in 
compliance with W3C spec. And that allows cross-domain communication from 
the browser. All responses have a wildcard same-origin which makes them 
completely public and accessible to everyone, including any code on any site.

# Authentication

<SecurityDefinitions />
        "#;
        openapi.info.description = Some(desc.to_owned());
    }
}

struct SecuritySchemas;
impl Modify for SecuritySchemas {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let components = match openapi.components.as_mut() {
            Some(c) => c,
            None => {
                openapi.components = Some(Components::new());
                openapi.components.as_mut().unwrap()
            },
        };
        let flows = [Flow::Password(
            Password::new("http://localhost:8080/realms/default/protocol/openid-connect/token", Scopes::default())
        )];
        let oauth2 = OAuth2::new(flows);
        let scheme = SecurityScheme::OAuth2(oauth2);
        components.add_security_scheme("fpa-security", scheme);
    }
}