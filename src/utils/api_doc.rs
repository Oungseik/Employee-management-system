use utoipa::{
    openapi::security::{ApiKey, ApiKeyValue, SecurityScheme},
    Modify, OpenApi,
};
use utoipauto::utoipauto;

#[utoipauto]
#[derive(OpenApi)]
#[openapi( modifiers(&SecurityAddon))]
pub struct ApiDoc;

pub struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(component) = openapi.components.as_mut() {
            component.add_security_scheme(
                "Authorization",
                SecurityScheme::ApiKey(ApiKey::Header(ApiKeyValue::new("Authorization Header"))),
            );

            component.add_security_scheme(
                "auth_token",
                SecurityScheme::ApiKey(ApiKey::Cookie(ApiKeyValue::new("auth_token"))),
            );
        }
    }
}
