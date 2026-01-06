use actix_web::{get, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct HelloResponse {
    pub message: String,
    pub number: i32,
    //pub value: String,
}

#[utoipa::path(
    get,
    path = "/hello",
    tag = "Test",
    responses(
        (status = 200, description = "Hello world response", body = HelloResponse)
    )
)]
#[get("/hello")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().json(HelloResponse {
        message: "Hello, world!".to_string(),
        number: 42,
        //value: "pupupu".to_string(),
    })
}
