use openapi_client_generator::{generate_library, GenerateLibrary};

fn main() {
    generate_library(GenerateLibrary {
        name: "Plaid".to_string(),
        yaml_path: "openapi.yaml".into(),
        dest_path: "src".into(),
        lib_rs_path: Some("template/lib.rs".into()),
        model_rs_path: Some("template/model.rs".into()),
    }).unwrap();
}