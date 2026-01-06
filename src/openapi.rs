use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    info(
        version = "0.1.0",
        title = "Test server",
        license(identifier = "MIT"),
    ),
    paths(
        crate::api::hello,
    ),
    components(),
    tags(
        (name = "Test", description = "Test endpoints")
    ),
)]
pub struct ApiDoc;
