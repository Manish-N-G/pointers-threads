use clap;

// Parser is a marco that matches arguements passed to a binary
// with what is defined for our application. Basically generates
// and argumenet parser.
#[derive(Debug, clap::Parser)]
// This configures some metadate for our application
//name = "pointers_threads",
#[command(version, about = "Pointers and threads application")]
pub struct ArgsPointersThreads {
    /// this flag chooses to enable logging
    /// group="something" // could also work
    #[arg(long, conflicts_with = "module_run")]
    logs: bool,

    /// Write logs to the terminal
    // #[arg(long, requires = "logs", conflicts_with = "out_file")]
    #[arg(long, requires = "logs")]
    cli: bool,

    /// Write logs to a file
    #[arg(
        long,
        requires = "logs",
        // conflicts_with = "cli",
        require_equals = false,
        default_missing_value = ""
    )]
    out_file: Option<std::path::PathBuf>,

    #[arg(long, requires = "logs", require_equals = true)]
    level: Option<String>,

    // Run future modules
    // Run a module (placeholder for future functionality)
    #[arg(long, conflicts_with = "logs")]
    module_run: bool,

    // #[command(subcommand)] // to have logs not --logs
    // the option makes it an option of None or Some(Command)
    // Here is should extend that we could have additional
    // arguements or not
    // command: Option<Commands>,
}

impl ArgsPointersThreads {
    pub fn get_log(&self) -> bool {
        self.logs
    }

    pub fn get_cli(&self) -> bool {
        self.cli
    }

    // should this be mutable? //todomanish:
    pub fn get_out_file(&self) -> &Option<std::path::PathBuf> {
        &self.out_file
    }

    pub fn get_level(&self) -> crate::logging::LogLevel {
        crate::logging::get_level(&self.level) 
    }

    pub fn get_module(&self) -> bool {
        self.module_run
    }
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
