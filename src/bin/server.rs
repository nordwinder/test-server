use actix_web::{App, HttpServer};
use test_server::api::hello;
use test_server::openapi::ApiDoc;
use utoipa::OpenApi;

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
