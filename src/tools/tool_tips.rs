//! List of tools that helps to integrate this lib better
//!
//! This doesn't necessarily mean that these tools are only for pointers, threads and async functions,
//! but It should encompass general testing, benching, performance options. Not to mention other
//! specific traits that will help to.
//!
//! <br><br>
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
//! <br><br>
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
//! - Cargo Seek: Its a pretty TUI that is use to search, add and info
//! among others. Its a nice tool that helps us manage it all in one place.
//! To use it, do the following
//! ```text
//! cargo seek
//! cargo seek --help
//! ```
//! <img src="https://github.com/Manish-N-G/pointers-threads/blob/master/assets/seek.png?raw=true"></img>
//!
//! - Cargo Clippy: Clippy is quite powerful to linting and ignoring files that we want to
//! ignore for linking errors. To make it work, we could provide some files to be ignored
//! or linting. This is achieved via the `Cargo.toml` file and 'Clippy.toml' file. However
//! they serve different purposes by default. See the online documentation to know more.
//! - [`Clippy`](https://doc.rust-lang.org/nightly/clippy/index.html) documentation link.
//! ```text
//! cargo clippy
//! ```
//!
//! <br><br>
//! ### Testing Tools
//! - Cargo tests and Cargo doc --test are some of the ways we can do testing, and we can
//! test based on the path as well if needed.
//! ```text
//! cargo test
//! cargo doc --test
//! cargo test -- --list // to list test functions
//! ```
//! However. there are some newer tools that are available to use that can make this
//! easier for use to do.
//! - Cargo NexText
//! Using NexText, we can not only speed of test, but also provides a cleaner interface
//! and with more time related details when testing. We can do this by looking at
//! some of the commands.
//! ```text
//! // currently for unit/integration tests
//! // not yet available for doc tests
//! cargo nextest run
//! ```
//! Nextest allows us to handle slow tests, automatically retry tests, marks heavy tests
//! separately, choose the run the serially or parallelly, allow us to use this as CI
//! runs. It give us test coverage, allows for mutation based testing, even going to the
//! point of observing system behaviour.
//!
//! <br><br>
//! ### Benching Tools
//! - We can use cargo bench to bench our code. This can be available with nextest as well
//! to see how long it takes to complete. However bench in generally through of the
//! default.
//! ```text
//! cargo bench
//! ```
//! However, there are other tools at our disposal, that can help us to perhaps get even
//! better results that using cargo bench.
//! - [`Criterion`](https://docs.rs/criterion/latest/criterion/) is another very known
//! package that gives us good test results and functionality that allows to test our
//! project.
//! ```text
//! // will use the same command, expect it now depends on the cargo criterion modules
//! cargo bench
//!
//! // before criterion, we would do something like
//! #![feature(test)]
//! extern crate test;
//! use test::Bencher;
//! use test::black_box;
//! use rayon::prelude::*;
//!
//! fn get_incremental_sum_from_index_count( idx: u16, count: u64 ) -> u128 {
//!     ((idx as u128)..=(idx as u128+count as u128)).into_par_iter().sum()
//! }
//! 
//! // we just aadd the tag:
//! #[bench]
//! fn normal_bench(b: &mut Bencher) {
//!     b.iter( || get_incremental_sum_from_index_count( 400, 423293393) );
//!     b.iter( || get_incremental_sum_from_index_count( black_box(303), black_box(23423439)) );
//! }
//!
//! ```
//! for criterion
//! ```
//! // this can actually be run, but we dont assert anything with it
//! use criterion::{criterion_group, criterion_main, Criterion};
//! use std::hint::black_box;
//! use rayon::prelude::*;
//!
//! fn get_incremental_sum_from_index_count( idx: u16, count: u64 ) -> u128 {
//!     ((idx as u128)..=(idx as u128+count as u128)).into_par_iter().sum()
//! }
//!
//! fn crit_bench(c: &mut Criterion) -> &mut Criterion {
//!     c.bench_function("inc to count from idx", |b| {
//!         b.iter( || get_incremental_sum_from_index_count( 400, 423293393) );
//!         b.iter( || get_incremental_sum_from_index_count( black_box(303), black_box(23423439)) );
//!     })
//! }
//!
//! // run these 2 to get the criterion bench
//! // cargo bench
//! // criterion_group!( bench_name, crit_bench );
//! // criterion_main!( bench_name );
//! ```
//!
//! <br><br>
//! ### Generate Templates for specific popular projects
//! - Cargo generate is another useful crate that allows us to make default template for
//! specific popular packages that are available from the internet. To get a good feel of this
//! create, have a look at some of the commands below
//! ```text
//! // ensure git is setup 1st
//! cargo generate --git git@github.com:rustwasm/wasm-pack-template.git --name mywasm
//!
//! // Generate a project with interactive prompts
//! cargo generate --git https://github.com/username/mytemplate.git
//!
//! // We could also setup git ssh and for ssh using
//! ~/.gitconfig
//! [url "ssh://git@github.com/"]
//! insteadOf = https://github.com/
//! ```
//! Basically we can use a list of templates that are available to have an initial setup
//! for our project and this is a good stepping stone to start something that we are not
//! familiar with, not to mention, have a checklist of what could be available for our
//! project. It scaffolds new projects by leveraging pre-existing git repositories.
//! Have a look the the documentation online for
//! [`Cargo Generate`](https://cargo-generate.github.io/cargo-generate/usage/git-over-ssh.html)
//!
//!
#![allow(clippy::doc_lazy_continuation)] // just to allow next line when we use -
