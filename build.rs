// Out custom build scipt for adding things to our lib
// This compiles first before building the rest of the package
fn main() {
    let logo_file = include_bytes!("assets/logo.png");
    println!("We have the logo file: {:?}",logo_file);
}
