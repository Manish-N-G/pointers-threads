// Out custom build scipt for adding things to our lib
// This compiles first before building the rest of the package
// For info about build dependencies and printing, we could
// consider cargo_build. However, we are using base cargo instructions
// for our implementation

use std::env;
use std::path::PathBuf;
// use std::path::Path;
// use std::fs;
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpStream};
use std::process::Command;

fn command_warning_image(url: &str) {
    if !is_internet_available() {
        println!("cargo::warning=Internet issue for command_warning_image");
        return
    }

    let output = Command::new("curl")
        .arg("-s") // silent
        .arg("-I") // HEAD request. Fetches only headers
        .arg("-o") // rediret output
        .arg("/dev/null") // output location, so we dont worry about deleting it in null
        .arg("-w") // write out
        .arg("%{http_code}") // what curl should print. I need this or else it will fail
        .arg(url)
        .output();
    // eg: "curl -s -I -o /dev/null -w "%{http_code}" https://www.google.com"

    let result: (bool, String) = match output {
        Ok(out) => {
            // Check if the command succeeded and the status code is 200
            // Converts slice of byte to string for from_utf8 lossy
            let output_status = String::from_utf8_lossy(&out.stdout);
            (out.status.success(), output_status.into())
        }
        _ => (false, "Err".into()),
    };

    if result.0 {
        // println!("cargo metadata=Image link is successful {}", result.1);
        println!("cargo::warning=link is successful {}", result.1);
    } else {
        println!("cargo::warning=Link address: {url}"); // cargo warning message
        println!("cargo::warning=curl exit status: {:?}", result.1);
        panic!("cargo:error=Link is not active: {}", result.1);
    }
}

// Just for mac/linux
// fn has_internet() -> bool {
//     // Linux/macOS example
//     let output = Command::new("ping")
//         .args(["-c", "1", "8.8.8.8"])
//         .output();
//
//     match output {
//         Ok(o) => o.status.success(),
//         Err(_) => false,
//     }
// }

use url::Url;
fn check_url_link(url_string: &str) -> bool {
    if !is_internet_available() {
        println!("cargo::warning=Internet url issue for has_url_link");
        return false
    }

    // 1. Parse the URL string
    let url = match Url::parse(url_string) {
        Ok(u) => u,
        Err(_) => return false,
    };

    // 2. Resolve URL to socket addresses (e.g., IP:Port)
    // The closure || None uses the default port for the scheme (80 for http, 443 for https)
    let addrs = match url.socket_addrs(|| None) {
        Ok(addrs) => addrs,
        Err(_) => {
            println!("cargo::warning, addrs error for url socket {}", url);
            return false;
        }
    };

    // 3. Attempt to connect to the first available address
    // If connection succeeds, internet is likely available
    if TcpStream::connect(addrs.first().unwrap()).is_ok() {
        println!("cargo::warning=link is successful here");
        true
    } else {
        println!(
            "cargo::warning, Could not connect to tcpstream addrs for {}",
            addrs.first().unwrap()
        );
        false
    }
}

fn is_internet_available() -> bool {
    // Connect to Google's DNS server on port 53 or HTTP on port 80
    // TcpStream::connect("8.8.8.8:53").is_ok() // also works, but no time out

    let socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(8, 8, 8, 8)), 53);
    TcpStream::connect_timeout(&socket, std::time::Duration::from_secs(2)).is_ok()
}

fn assert_path_and_internet(
    var_file_path: &str,
    img_url_link: &str,
    image_type: &str,
) -> Result<String, ()> {
    // Fetches the environment variable `key` from the current process.
    // For us, located in .cargo/config.toml file
    let var_link_value: String = env::var(var_file_path).unwrap();

    // performs compile time check. We get value like /assets/logo_transparent
    assert!(PathBuf::from(&var_link_value).exists());

    let img_url: String = env::var(img_url_link).unwrap();
    if is_internet_available() {
        println!("cargo::warning=url for {}={:?}", image_type, img_url);
        Ok(img_url)
    } else {
        println!(
            "cargo::warning= NO Internet while checking: for {} -> url \"{}\"",
            image_type, img_url
        );
        Err(())
    }
}

fn main() -> Result<(), ()> {
    // let _logo_file = include_bytes!("assets/logo.png");
    let logo_var = "LOGO_PATH"; // local path
    let logo_link = "LOGO_URL";
    #[allow(unused)]
    let logo_url = assert_path_and_internet(logo_var, logo_link, "logo")?;

    let bacon_var = "BACON_PATH"; // local path
    let bacon_link = "BACON_URL";
    #[allow(unused)]
    let bacon_url = assert_path_and_internet(bacon_var, bacon_link, "bacon")?;

    let seek_var = "SEEK_PATH"; // local path
    let seek_link = "SEEK_URL";
    #[allow(unused)]
    let seek_url = assert_path_and_internet(seek_var, seek_link, "seek")?;

    // cant pass logo_var into env!. I would have to use "LOGO_PATH" directly
    // env! takes string literals not variables
    // let logo_path:&str = env!(logo_var);  // wont work
    // let logo_path:&str = env!("LOGO_PATH");  // works

    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap_or_else(|_| {
        // Fallback if not set (rare in modern Cargo)
        env::var("TARGET").unwrap_or("unknown".to_string())
    });

    if target_os.contains("linux") {
        println!("cargo::rustc-cfg=platform_linux");
        command_warning_image(&logo_url);
        command_warning_image(&bacon_url);
        command_warning_image(&seek_url);
    } else if target_os.contains("windows") {
        println!("cargo::rustc-cfg=platform_windows");
        check_url_link(&logo_url);
        check_url_link(&bacon_url);
        check_url_link(&seek_url);
    } else if target_os.contains("macos") {
        println!("cargo::rustc-cfg=platform_macos");
        command_warning_image(&logo_url);
        command_warning_image(&bacon_url);
        command_warning_image(&seek_url);
    } else {
        // todo:
    }

    /*  Could do something like this
    We test just for linux

    #[cfg(platform_windows)]
    fn setup() { /* Windows logic */ }

    #[cfg(platform_linux)]
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
    println!("cargo::rerun-if-env-changed={}", logo_var);
    println!("cargo::rerun-if-env-changed={}", logo_link);
    println!("cargo::rerun-if-env-changed={}", logo_url);
    println!("cargo::rerun-if-changed=build.rs");
    println!("cargo::rerun-if-changed=cargo.toml");
    Ok(())
}
