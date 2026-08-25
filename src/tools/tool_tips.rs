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
