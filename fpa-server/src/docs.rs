
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
    ),
    components(schemas(
        crate::model::page::Page,
        crate::model::users::Model,
        crate::model::projects::Model,
    )),
    modifiers(&SecuritySchemas),
)]
pub struct ApiDoc;    

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