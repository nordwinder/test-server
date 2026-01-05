use std::fs::File;
use std::io::Write;

use test_server::openapi::ApiDoc;
use utoipa::OpenApi;

fn main() -> std::io::Result<()> {
    let openapi = ApiDoc::openapi();

    let yaml = openapi
        .to_yaml()
        .expect("Failed to convert OpenAPI to YAML");

    let mut file = File::create("openapi.yaml")?;
    file.write_all(yaml.as_bytes())?;

    println!("openapi.yaml generated");

    Ok(())
}
