use utoipa::OpenApi;
use utoipauto::utoipauto;

#[utoipauto]
#[derive(OpenApi)]
#[openapi(tags(
    (name = "Employee management", description = "Service to manage employees")
))]
pub struct ApiDoc;
