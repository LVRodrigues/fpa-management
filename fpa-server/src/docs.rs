use serde_json::json;
use utoipa::{
    openapi::{
        extensions::Extensions,
        security::{Flow, OAuth2, Password, Scopes, SecurityScheme},
        Components,
    },
    Modify, OpenApi,
};

#[derive(OpenApi)]
#[openapi(
    tags(
        [name = "Status", description = "Check service health."],
        [name = "Projects", description = "FPA Projects management."],
        [name = "Empiricals", description = "Empiricals Adjustments Factors."],
        [name = "Factors", description = "Adjustments Factors."],
        [name = "Frontiers", description = "Analysis frontier."],
        [name = "Functions", description = "Function management for analysis."]
    ),
    paths(
        crate::handlers::health,
        crate::handlers::projects::list,
        crate::handlers::projects::by_id,
        crate::handlers::projects::create,
        crate::handlers::projects::update,
        crate::handlers::projects::remove,
        crate::handlers::empiricals::list,
        crate::handlers::empiricals::update,
        crate::handlers::factors::list,
        crate::handlers::factors::update,
        crate::handlers::frontiers::list,
        crate::handlers::frontiers::by_id,
        crate::handlers::frontiers::create,
        crate::handlers::frontiers::update,
        crate::handlers::frontiers::remove,
        crate::handlers::functions::list,
        crate::handlers::functions::by_id,
        crate::handlers::functions::create,
        crate::handlers::functions::update,
        crate::handlers::functions::remove,
    ),
    components(
        schemas(
            crate::model::sea_orm_active_enums::EmpiricalType,
            crate::model::sea_orm_active_enums::FactorType,
            crate::model::sea_orm_active_enums::InfluenceType,
            crate::model::sea_orm_active_enums::FunctionType,
            crate::model::users::Model,
            crate::model::projects::Model,
            crate::model::empiricals::Model,
            crate::model::factors::Model,
            crate::model::frontiers::Model,
            crate::model::versions::Model,
            crate::error::ErrorResponse,
            crate::handlers::projects::ProjectParam,
            crate::handlers::empiricals::EmpiricalParam,
            crate::handlers::factors::FactorParam,
            crate::handlers::frontiers::FrontierParam,
            crate::handlers::functions::FunctionALI,
            crate::handlers::functions::FunctionALIParam,
            crate::handlers::functions::FunctionAIE,
            crate::handlers::functions::FunctionAIEParam,
            crate::handlers::functions::FunctionEE,
            crate::handlers::functions::FunctionEEParam,
            crate::handlers::functions::FunctionCE,
            crate::handlers::functions::FunctionCEParam,
            crate::handlers::functions::FunctionSE,
            crate::handlers::functions::FunctionSEParam,
            crate::handlers::functions::FunctionData,
            crate::handlers::functions::Function,
            crate::handlers::functions::FunctionParam,
        ),
    ),
    modifiers(&SecuritySchemas, &InfoModifier),
)]
pub struct ApiDoc;

struct InfoModifier;
impl Modify for InfoModifier {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let value = json!({
            "url": "/assets/logo.png",
            "backgroundColor": "#FFFFFF"
        });
        openapi.info.extensions = Some(Extensions::builder().add("x-logo", value).build());

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
            }
        };
        let flows = [Flow::Password(Password::new(
            "http://localhost:8080/realms/default/protocol/openid-connect/token",
            Scopes::default(),
        ))];
        let oauth2 = OAuth2::new(flows);
        let scheme = SecurityScheme::OAuth2(oauth2);
        components.add_security_scheme("fpa-security", scheme);
    }
}
