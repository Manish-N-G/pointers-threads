use std::thread;
pub mod lib_th1a;
pub mod lib_th1b;

pub use lib_th1a::*;

/// Simple function that creates threads
///
/// Here the threads are run concurrently but we make sure that we
/// are joining then at the end to ensure that they fill
/// finish before this function gets done.
/// ```rust
/// use std::thread;
///
/// let th1 = thread::spawn(|| {
///     println!("th1 inside");
///     "th1"
/// });
/// let th2 = thread::spawn(|| {
///     println!("th2 inside");
///     "th2"
/// });
/// assert_eq!("th1", th1.join().unwrap());
/// assert_eq!("th2", th2.join().unwrap());
/// ```
/// This will join and assert the values
pub fn thread1st_get_current() -> thread::ThreadId {
    let th1 = thread::spawn(th1_print_id);
    let th2 = thread::spawn(th1_print_id);

    println!("Main thread after spanning this tread");
    println!("generally the threads start immediately");

    th1.join().unwrap();
    th2.join().unwrap();
    println!("Main thread id is {:?}", thread::current().id());
    th1_print_id()
}

fn th1_print_id() -> thread::ThreadId {
    thread::current().id()
}
