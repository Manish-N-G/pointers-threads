use pointers_threads::clap_file;
use clap::{self, Parser};


// Embeds the content of 'config.txt' (located at crate root) as a static string
const WELCOME_DATA: &str = include_str!("../assets/welcome.txt");

pub fn welcome_message() {
    let mut welcome = WELCOME_DATA.split(r#"""#); // same as "\"\"\""
    if let Some(s) = welcome.find(|val| val.contains("welcome")) {
        println!();
        for line in s.lines().skip(1) {
            println!("{line}");
        }
    }
    if let Some(s) = welcome.find(|val| val.contains("label3")) {
        s.lines().skip(1).for_each( |line| println!("{line}"));
        println!("----------------------\n");
    }
}

// pub fn args_parser(args: &str) -> Result<Vec<clap_file::> {
pub fn args_parser() {
    // Note, we need to use clap here    
    
    // need to import Clap::Parser for this
    let args = clap_file::ArgsPointersThreads::parse();

    println!("args is {:?}", args); //todel
    if args.get_log() {
        match crate::logging::process_log_arg(&args) {
            // _ => todo!(),
            _ => println!("===we get process log args")
        }
    } else if args.get_module() {


    }
    // need to see how to handle errors here
    // todo!()
}
