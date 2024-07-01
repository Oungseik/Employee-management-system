use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(paths(crate::routers::check_health::check_health), components())]
pub struct ApiDoc;


