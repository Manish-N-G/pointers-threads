// Out custom build scipt for adding things to our lib
// This compiles first before building the rest of the package

use std::env;
use std::path::PathBuf;
// use std::path::Path;
// use std::fs;

use std::process::Command;

fn command_warning_image(url: &str) {
    let output = Command::new("curl")
        .arg("-s") // silent
        .arg("-I") // HEAD request. Fetches only headers
        .arg("-o") // rediret output
        .arg("/dev/null")  // output location
        .arg("-w") // write out
        .arg("%{http_code}")  // what curl should print. I need this or else it will fail
        .arg(url)
        .output();
    // eg: "curl -s -I -o /dev/null -w "%{http_code}" https://www.google.com"

    let result:(bool, String) = match output {
        Ok(out) => {
            // Check if the command succeeded and the status code is 200
            // Converts slice of byte to string for from_utf8 lossy
            let output_status = String::from_utf8_lossy(&out.stdout); 
            ( out.status.success() , output_status.into() )
        }
        _ => ( false, "Err".into() )
    };

    if result.0 {
        println!("Image link is successful {}", result.1);
    } 
    else {
        println!("cargo:warning=Link address: {url}"); // cargo warning message
        println!("cargo:warning=curl exit status: {:?}", result.1);
        panic!("cargo:error=Link is not active: {}", result.1);
    }
}

fn assert_path_and_url(var_file_path:&str, img_url_link: &str, image_type: &str) -> String {
    let var_link_value:String = env::var(var_file_path).unwrap();
    // performs compile time check. We get value like /assets/logo_transparent
    let img_exists_in_file = PathBuf::from(&var_link_value).exists();
    assert!(img_exists_in_file);

    let img_url:String = env::var(img_url_link).unwrap();
    println!("cargo:warning=img_url for {} = {:?}",image_type, img_url); // not cargo warn message
    img_url  
}

fn main() {
    // let _logo_file = include_bytes!("assets/logo.png");
    let logo_var = "LOGO_PATH";
    let logo_link = "LOGO_URL";
    #[allow(unused)]
    let logo_url = assert_path_and_url(logo_var, logo_link, "logo");

    let bacon_var = "LOGO_PATH";
    let bacon_link = "LOGO_URL";
    #[allow(unused)]
    let bacon_url = assert_path_and_url(bacon_var, bacon_link, "bacon");

    let seek_var = "LOGO_PATH";
    let seek_link = "LOGO_URL";
    #[allow(unused)]
    let seek_url = assert_path_and_url(seek_var, seek_link, "seek");

    // cant pass logo_var into env!. I would have to use "LOGO_PATH" directly
    // env! takes string literals not variables
    // let logo_path:&str = env!(logo_var);  // wont work
    // let logo_path:&str = env!("LOGO_PATH");  // works

    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    if target_os == "linux" {
        command_warning_image(&logo_url);
        command_warning_image(&bacon_url);
        command_warning_image(&seek_url);
        // Linux-specific build steps
        println!("cargo:rustc-cfg=build_target_linux");
    } else if target_os == "windows"{
        // we are not handling for windows
        // println!("cargo:rustc-cfg=build_target_windows");
    }

    /*  Could do something like this
    We test just for linux

    #[cfg(build_target_windows)]
    fn setup() { /* Windows logic */ }

    #[cfg(build_target_linux)]
    fn setup() { /* Linux logic */ }
    */

    
    // OUT_DIR already inbuild in rust
    // let out_dir = env::var("OUT_DIR").unwrap();
    // let dest_path = Path::new(&out_dir).join(&doc_name);
    // let dest_path = PathBuf::new().join(&out_dir).join(doc_name);// (&out_dir).join(doc_name);

    // Generate a Rust file with a constant or doc comment
    // let docs_code = format!(
    //     r#"
    //     /// This is the value of {} at compile time: {}
    //     pub const GENERATE_LOGO: &str = "../../../{}";
    //     "#,
    //     "our env var", logo_var, logo_path
    // );
    // fs::write(&dest_path, docs_code).unwrap();

    // println!("cargo:rustc-env={}={}", logo_var, logo_path );
    println!("cargo:rerun-if-env-changed={}", logo_var);
    println!("cargo:rerun-if-env-changed={}", logo_link);
    println!("cargo:rerun-if-env-changed={}", logo_url);
    println!("cargo::rerun-if-changed=build.rs");
    println!("cargo::rerun-if-changed=cargo.toml");
}


