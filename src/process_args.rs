use pointers_threads::clap_file;
use clap::{self, Parser};

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

    // todo!()
}
