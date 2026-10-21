use std::fmt::{self, Write};
use crate::clap_file::*;
use clap::Parser;

// pub struct Logger {
//     message: Vec<String>,
// }

#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum LogLevel {
    Info,
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

pub fn get_level(val: &Option<String> ) -> LogLevel {
    match val {
        Some(value) if value.to_lowercase() == "info" => LogLevel::Info,
        Some(value) if value.to_lowercase() == "trace" => LogLevel::Trace,
        Some(value) if value.to_lowercase() == "warn" => LogLevel::Warn,
        Some(value) if value.to_lowercase() == "error" => LogLevel::Error,
        _ => LogLevel::Info
    }
}

    // let args = clap_file::ArgsPointersThreads::parse();
    //     match process_log_arg(&args) {
    //         _ => todo!(),
    //     }
    //

// This whole file needs to be done using makefile as well. //todomanish:
pub fn process_log_arg(args: &ArgsPointersThreads) -> Result<(), String> {
    if args.get_cli() {
        // here we endup making what we need to make sure that we are able to 
        // display all this in the command line
        // I will end up using a formatter to make this happen
        // todo!() // make cli logs
        println!("===we have cli");
    } 
    if let Some(val) = args.get_out_file() {
        // this is where we have to dynamically see what we need to do
        // in order to get the output files.
        // I want to have the datetime option. Otherwise we will use 
        // one that is given in commandline arguement
        match create_dir() {
            // _ => todo!()
            _ => println!("===we have create dir"),
        }
    }

    Ok(())
}

// eg:
// myapp // works
// myapp --logs // works
// myapp --logs --cli // works
// myapp --cli // doesnt work
// myapp --logs --out_files // works
// myapp --logs --out_files="hello" // works
// myapp --logs --out_files "hello" // works?
// myapp --logs --out_files  hello  // works?
// myapp --out_files // doesnt works
