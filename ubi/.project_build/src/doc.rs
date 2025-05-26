use crate::server::api::*; 
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    info(
        title = "ubi",
        description = "this is my app",
        version = "0.1"
    ),
    paths(crate::server::api::get, crate::server::api::post),
    components(schemas(produk, produk)),
    tags()
)]
pub struct ApiDoc;