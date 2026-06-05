// This is anohter addition of doc comments on top of what is 
// already mentioned before. //! is for lib_th1a again. And its
// the root of this crate. /// Will need to be appended only to
// another module under this or we will have to use it on functions
// which will show up the description in the doc file.
//!
//! These select few functions is mainly used to thread creation
//! and errors that could be encountered when creating threads.
use std::thread;

// Remember, rust docs are not compiled inside lib_th1a
// but turns it into a separate crate.
/*
/// Creates a spawned thread and Joins and returns the result
///
/// This is a simple test thread that takes a vector, and adds 
/// 6 to it at the end of the vec
/// ```
/// use pointers_threads::lib_th1a::*;
/// use std::thread;
/// let v = vec![1,2,3,4,5];
/// let a1 = thread1a_add6(v);
///
/// // we cant do print statement event with --show-output/ --nocapture
/// assert_eq!(&a1, &[1,2,3,4,5,6])
/// ```
*/
pub fn thread1a_add6(mut v: Vec<i32>) -> Vec<i32> {
    // println!("Testing th1a thread1a");
    // v is something like v = vec![1,2,3,4,5]; // this works
    // Here rust uses "move closure inference". This means 
    // that rust automatically decided to move without explicitly 
    // writing it.
    // If we need to pass references, it needs to have static
    // livetime in general
    thread::spawn(|| { // move is made automatically
        v.push(6);
        for x in v.iter() {
            println!("x is {x:?}");
        }
        v
    }).join().unwrap()
    // if we were to use v again, we will have borrow issues
    // println!("v outside is {v:?}"); // this will cause problems
}


pub fn thread1a_error() {
    let v = vec![1,2,3];
    // this will throw error as it needs to move, or static lifetime
    // thread::spawn(|| {
    //     println!("{:?}", v);   // only &v needed
    // }); // ← ERROR! Needs `move` because v would be borrowed, but thread may outlive it

    thread::spawn(move|| {
        println!("v is {:?}",v );
    });
}
