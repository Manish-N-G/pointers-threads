//! This lib has a collection of pointers and thread type functions availalbe
//!
//! This covers a lot of types. One is that we try to implement our own type
//! of pointers and test it with what the std uses. Its not exactly the one
//! in the std lib, but it gives a feel for how they operate. They wont work
//! exactly the same as well.
//!
//! Along with pointers, we also see that we have concurrently type functions
//! that allow us to know how this works. This creates differet threads and
//! uses different libs to test and see what works and how it works.
//!
//! We also cover async that shows how futures and async operate, sometimes with
//! multithreading/tasks, under the hood.

//! These module covers how different type of threads and pointers are used
//!
//! In order to understand threads, we will also have to worry about
//! pointers and what pointer is best to use. Hence all this is accessable
//! in these module.
//!
//! We will also be looking into Async and how they work and understand how
//! operates under the hood. In a way, Our goal is to know to we can have
//! concurrent and Async operations and how to use them.
//!
//! We can also log and analyse our functions and this are achivable for some 
//! other libs that are present in this library.
//!

#![doc(html_playground_url = "https://play.rust-lang.org/")]
// not pub, dont need to create doc type here
pub mod threads;
pub mod pointers;

/*
//NOTE: This is not good practice to add doc comment here like this. Its better
//to create the doc comments at the start of the module/file in order to not attach
//unnecessary docs that could leak to functions

// NOTE: Even if we e dont use //! ,we should not use it here, as its not the 1st line

/// This module covers how different type of threads and pointers are used
///
/// In order to understand threads, we will also have to worry about
/// pointers and what pointer is best to use. Hence all this is accessable
/// in thei module.
///
*/
pub use threads::*;
pub use pointers::*;
