extern crate cbindgen;

use std::env;

const CONFIG_FILE_PATH: &str = "./cbindgen.toml";
const GENERATED_HDR_PATH: &str = "../include/bindings.h";

fn main() {
    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=build.rs");

    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    let config =
        cbindgen::Config::from_file(CONFIG_FILE_PATH).expect("couldn't read cbindgen.toml");
    let root_namespace = config.namespace.clone();

    cbindgen::Builder::new()
        .with_crate(crate_dir)
        .with_config(config)
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(GENERATED_HDR_PATH);

    let result = add_forward_decl(root_namespace);
    match result {
        Ok(_) => (),
        Err(err_msg) => println!(
            "adding forward declarations for structures failed: {}",
            err_msg
        ),
    }
}

fn add_forward_decl(root_namespace: Option<String>) -> Result<(), String> {
    if let Some(namespace) = root_namespace {
        let target = format!("namespace {} {{\n", namespace);

        println!("{target}");

        let declarations_to_forward = ["struct Expr;"];
        let concatenated_declarations = declarations_to_forward.concat();

        let contents = std::fs::read_to_string(GENERATED_HDR_PATH)
            .map_err(|e| format!("failed to read generated header file: {}", e.to_string()))?;

        let offset = contents.find(&target).ok_or("root namespace not found.")?;

        let prefix = &contents[0..offset + target.len()];
        let suffix = &contents[offset + target.len()..];
        let content_after_insertion: String = format!(
            "{}\n{}\n{}",
            prefix.to_string(),
            concatenated_declarations,
            suffix.to_string()
        );

        let _ = std::fs::write(GENERATED_HDR_PATH, content_after_insertion)
            .map_err(|e| format!("failed to write modified file: {}", e.to_string()))?;
    }
    Ok(())
}
