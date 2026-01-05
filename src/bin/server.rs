use actix_web::{get, App, HttpResponse, HttpServer, Responder};

use test_server::openapi::{ApiDoc, HelloResponse};
use utoipa::OpenApi;

#[get("/hello")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().json(HelloResponse {
        message: "Hello, world!".to_string(),
    })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let openapi = ApiDoc::openapi();
    let addrs = ("127.0.0.1", 9000);
    println!("Hello at http://{}:{}/hello", addrs.0, addrs.1);
    println!("Swagger at http://{}:{}/swagger/", addrs.0, addrs.1);

    HttpServer::new(move || {
        App::new().service(hello).service(
            utoipa_swagger_ui::SwaggerUi::new("/swagger/{_:.*}")
                .url("/api-doc/openapi.json", openapi.clone()),
        )
    })
    .bind(addrs)?
    .run()
    .await
}
