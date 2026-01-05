use serde::{Deserialize, Serialize};
use utoipa::{OpenApi, ToSchema};

#[derive(Serialize, Deserialize, ToSchema)]
pub struct HelloResponse {
    pub message: String,
}

#[utoipa::path(
    get,
    path = "/hello",
    tag = "Test",
    responses(
        (status = 200, description = "Hello world response", body = HelloResponse)
    )
)]
pub async fn hello_handler() {}

#[derive(OpenApi)]
#[openapi(
    paths(
        hello_handler
    ),
    components(
        schemas(HelloResponse)
    ),
    tags(
        (name = "Test", description = "Test endpoints")
    )
)]
pub struct ApiDoc;
