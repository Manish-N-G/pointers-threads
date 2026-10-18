// use paste::paste;
//
// #[allow(unused_macros)]
// macro_rules! my_func {
//     // 'm' is for literal matching.
//     // ident can be used here more for alphabets. cant start with a number
//     // but ident can have numbers present else where.
//     ( 'm', $md:literal, $($va:ident),* ) => {{ $(
//             paste! {
//                 [<mod $md _ $va>]();
//             }
//         )*
//     }};
//     ( 't', $nb:literal, "", $va:literal ) => {{
//         paste! {
//             [<th$nb>]::[<thread $va>]();
//         }
//     }};
//     ( 't', $nb:literal, $nest: literal, $va:literal ) => {{
//         paste! {
//             [<th $nb>]::[<th $nest>]::[<thread $va>](); }
//     }};
// }

use std::fmt::{self, Write};

pub struct Logger {
    message: Vec<String>,
}

#[non_exhaustive]
pub enum LogLevel {
    Trace,
    Warn,
    Error,
}

fn create_dir() -> Result<(), std::io::ErrorKind> {
    let path = std::path::Path::new("/LogOutput");
    // if !path.is_dir() { // can avoid this with looking at errorkind

    // Filesystem is inherently fallable, which is why we have result type.
    // could be related to permissions, diskspace or path etc
    // we want a panic is we dont find it
    let Err(e) = std::fs::create_dir(path) else{
        return Ok(())
    };
    match e.kind() {
        std::io::ErrorKind::AlreadyExists => Ok(()),
        _ => Err(e.kind()),
    }
}

fn process_log_arg(args: &str) -> Result<> {
    // Note, we need to use clap here    
    if args.contains("out_logs") {
        match create_dir() {
            
        }
    } else if args.contains("cli_logs") {

    }

    todo!()
}



