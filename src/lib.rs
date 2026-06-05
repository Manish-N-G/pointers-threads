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

// not pub, dont need to create doc type here
mod threads1;

// We dont use //! and should not use it here, its its not the 1st line
/// This module covers how different type of threads and pointers are used
/// 
/// In order to understand threads, we will also have to worry about 
/// pointers and what pointer is best to use. Hence all this is accessable
/// in thei module.
///
pub use threads1::*;


