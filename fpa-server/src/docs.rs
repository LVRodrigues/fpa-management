use serde_derive::Serialize;
use utoipa::{openapi::{security::{Flow, OAuth2, Password, Scopes, SecurityScheme}, Components}, Modify, OpenApi, ToSchema};

/* #region Arrays */

///
/// A redeclaração da estrutura Project é apenas para renomear o tipo na montagem da Matriz Projects.
/// O tipo válido para operação no sistem está em model::projects::Model, que é renomeado
/// para Project na documentação da API.
/// 
/// No módulo model estão os tipos corretos gerados pelo cliente sea-orm-cli
/// 

#[derive(Debug, Serialize)]
struct Project;

#[derive(Debug, Serialize, ToSchema)]
struct Projects(Vec<Project>);

#[derive(Debug, Serialize)]
struct User;

#[derive(Debug, Serialize, ToSchema)]
struct Users(Vec<User>);

///
/// Ficou, por enquanto, como curiosidade. 
/// 
/// Use o objeto Page para recuperar listas.
/// 

/* #endregion */

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
        Users,
        crate::model::projects::Model,
        Projects
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