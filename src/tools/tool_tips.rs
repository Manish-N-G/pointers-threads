//! List of tools that helps to integrate this lib better
//!
//! This doesn't necessarily mean that these tools are only for pointers, threads and async functions,
//! but It should encompass general testing, benching, performance options. Not to mention other
//! specific traits that will help to.
//!
//! ### Spelling checking
//!
//! - For Spelling we could use [Cargo Spellcheck](http://crates.io/crates/cargo-spellcheck)
//! ```text
//! cargo spellcheck check
//! cargo spellcheck --fix
//! cargo spellcheck fix
//! cargo spellcheck fix "file_path"
//! ```
//! We can add them to our `cargo.toml` file
//! ```text
//! // for Cargo.toml
//! [package.metadata.spellcheck]
//! config = ".config/spellcheck.toml"
//! ..
//! ..
//! // in .config/spellcheck.toml
//! # Also take into account developer comments
//! dev_comments = true
//! skip_readme = false
//! // see full list in the git repo
//! ```
//!
//! ### Some Cargo based tools
//! - Cargo info and search: A tool to find out about a cargo package
//! To simply use it as follows. cargo info however, can give use what 
//! features are available to the package.
//! ```text
//! cargo info bacon
//! cargo search bacon
//! ```
//! - Cargo bacon: A tool to run all the different cargo tools.
//! We can use test, runs, clippy, pedantic warnings and others.
//! ```text
//! bacon
//! bacon --help
//! ```
//! <img src="https://github.com/Manish-N-G/pointers-threads/blob/master/assets/bacon_help.png?raw=true"></img>
//!
//! - Cargo Seek: Its a pritty TUI that is use to search, add and info
//! among others. Its a nice tool that helps us manage it all in one place.
//! To use it, do the following
//! ```text
//! cargo seek
//! cargo seek --help
//! ```
//! <img src="https://github.com/Manish-N-G/pointers-threads/blob/master/assets/seek.png?raw=true"></img>
//!
//! - Cargo Clippy: Clippy is quite powerful to linting and ignoring files that we want to 
//! ingore for linking errors. To make it work, we could provide some files to be ignored
//! or linting. This is achieved via the `Cargo.toml` file and 'Clippy.toml' file. However
//! they serve different purposes by default. See the online documentation to know more.
//! - [`Clippy`](https://doc.rust-lang.org/nightly/clippy/index.html) documentation link.
//! ```text
//! cargo clippy
//! ```
//!
//!
#![allow(clippy::doc_lazy_continuation)]  // just to allow nextline when we use -
