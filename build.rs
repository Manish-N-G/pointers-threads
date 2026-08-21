// Out custom build scipt for adding things to our lib
// This compiles first before building the rest of the package

use std::env;
use std::path::PathBuf;
use std::path::Path;
use std::fs;

fn main() {
    // let _logo_file = include_bytes!("assets/logo.png");
    let logo_var = "LOGO_PATH";
    let doc_var = "GEN_LOGO_FILE";

    // cant pass logo_var into env!. I would have to use "LOGO_PATH" directly
    // let logo_path:&str = env!(logo_var);
    let logo_path:String = env::var(logo_var).unwrap();
    let logo_exists = PathBuf::from(&logo_path).exists(); // compile time check
    assert!(logo_exists);

    let doc_name:String = env::var(doc_var).unwrap();
    
    // OUT_DIR already inbuild in rust
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join(&doc_name);
    // let dest_path = PathBuf::new().join(&out_dir).join(doc_name);// (&out_dir).join(doc_name);

    // Generate a Rust file with a constant or doc comment
    let docs_code = format!(
        r#"
        /// This is the value of {} at compile time: {}
        pub const GENERATE_LOGO: &str = "../../../{}";
        "#,
        "our env var", logo_var, logo_path
    );
    fs::write(&dest_path, docs_code).unwrap();

    println!("cargo:rustc-env={}={}", "GENNAME", doc_name );
    println!("cargo:rustc-env={}={}", logo_var, logo_path );
    println!("cargo:rerun-if-env-changed={}", logo_var);
    println!("cargo:rerun-if-env-changed={}", doc_var);
    println!("cargo::rerun-if-changed=build.rs");
    println!("cargo::rerun-if-changed=cargo.toml");
}


