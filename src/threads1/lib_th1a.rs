//
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

/// Creates a spawned thread and Joins handle the returns a vec with 42.
///
/// This is a simple test thread that takes a vector, and adds the value
/// 42 to it at the end of the vec
/// ```
/// use pointers_threads::lib_th1a::*;
/// use std::thread;
///
/// let v = vec![1,2,3,4,5];
/// let a1 = thread1a_add_42(v);
///
/// // we cant do print statement event with --show-output/ --nocapture
/// assert_eq!(&a1, &[1,2,3,4,5,42])
/// ```
pub fn thread1a_add_42(mut v: Vec<i32>) -> Vec<i32> {
    // println!("Testing th1a thread1a");
    // v is something like v = vec![1,2,3,4,5]; // this works
    // Here rust uses "move closure inference". This means 
    // that rust automatically decided to move without explicitly 
    // writing it.
    // If we need to pass references, it needs to have static
    // livetime in general
    thread::spawn(|| { // move is made automatically
        v.push(42);
        v
    }).join().unwrap()
    // if we were to use v again, we will have borrow issues
    // println!("v outside is {v:?}"); // this will cause problems
}

/// Creates a spawned thread and Joins handle the returns a vec modified with a value.
///
/// This is a simple test thread that takes a vector, and adds a given value
/// to it at the end of the vec
/// ```
/// use pointers_threads::lib_th1a::*;
/// use std::thread;
///
/// let v = vec![1,2,3,4,5];
/// let val = 10;
/// let a1 = thread1a_add_val(v, val);
///
/// // we cant do print statement event with --show-output/ --nocapture
/// assert_eq!(&a1, &[1,2,3,4,5,val])
/// ```
pub fn thread1a_add_val(mut v: Vec<i32>, val: i32) -> Vec<i32> {
    // closure might outlive the borrowed val. So we will move it in the closure
    // earlier, closure implicitely moved v in the closure, but with val in the
    // mix, we have to move it all in the closure.
    // this happens because we need static borrows only for spawned threads in 
    // isolation.
    thread::spawn( move|| { 
        v.push(val);
        v
    }).join().unwrap()
}

/// Spawned thread needs to have move when taking references
///
/// When creating a arr, we have to use move in the closue cause the thread
/// takes a reference, which cannot satisfy the 'static lifetime, and hence 
/// required the closure to move the reference. Not implicit move can be called
/// here cause we closure would only captured a reference.
/// ```
/// use pointers_threads::lib_th1a::*;
/// use std::thread;
///
/// let a1 = thread1a_move_issue();
///
/// // we cant do print statement event with --show-output/ --nocapture
/// assert_eq!(a1, 3);
/// ```
pub fn thread1a_move_issue() -> i32 {
    let v = vec![1,2,3];
    // this will throw error as it needs to move, or static lifetime
    // thread::spawn(|| {
    //     println!("{:?}", v);   // only &v needed
    // }); // ← ERROR! Needs `move` because v would be borrowed, but thread may outlive it

    thread::spawn( move || { // no implicit move. But this value is moved
        _ = &v;
        // drop(v); // adding this will compile without move, and makes move implicit
        3
    }).join().unwrap()

    // println!("v is {:?}", v);

}


/// Using thread builder with name to create a thread and send back a Vec.
///
/// When we use this thread builder, we created this thread builder type,
/// And then this allows us to add some attributes like name and stack size
/// than help define out thread a bit more. This is what we need as a default
/// to unsure that we have more control, but this is not always required
/// and adds some extra steps. To build a thread builder, we do the following
/// ```
/// use std::thread;
/// use pointers_threads::lib_th1a::*;
///
/// let val_from_builder = thread1a_builder( vec![1,2,3,4,5], false);
/// assert_eq!(&val_from_builder, &[1,2,3,4,5]);
///
/// ```
pub fn thread1a_builder<T>(val: Vec<T>, print_val: bool) -> Vec<T> 
where T: Sized + Send + std::fmt::Debug + 'static{
    // this thread is created using a basic thread builder type
    // with an attribute name with it. Here this is basic but gives
    // us the idea that cretain attributes can be added like name
    // stack size and others.
    let builder = thread::Builder::new().name("puchi".into());

    let handler = builder.spawn( move || {
        if print_val {
            println!("checking with name");
            println!("Thread builder printed of type T for {:?}", val);
        }
        val
    });

    handler.expect("Handler failed").join().expect("Join failed")
}

/// This is static function where we create a string literal with
/// the function. It doesnt do anything but maybe send the thread out
/// of the function
///
/// This will just say that we have a stat thread here. Notice that
/// we dont need to use move here, as we can use static reference
/// for the closure, even if its defined in the function. This is 
/// possible because the 'static lifetime exists for the life of 
/// this executable/lib
/// ```
/// use pointers_threads::lib_th1a::*;
/// use std::thread;
///
/// assert_eq!(thread1a_stat(), "this is stat thread");
/// ```
pub fn thread1a_stat() -> &'static str{
    let stat = "this is stat thread!"; // static lifetime
    thread::spawn(|| {
        // Instead of taking a variable, where I would have to move 
        // Into the closure, I will just check lenght and use that to 
        // decice if we print the value of not
        if stat.len().is_multiple_of(2) {
            // Here stat is not moved but referenced in this closure
            for x in stat.chars() {
                // possible as its static lifetime and chars borrows self
                print!("{} ", x);
            }
            println!();
        }

    }) .join() .unwrap();

    // this will allow me to use stat after calling it here
    if stat.len().is_multiple_of(2) {
        println!("stat val inside the function is: {stat}");
    }
    stat
}

/// This function shows that we need to use move even when we pass
/// a 'static str as move only would only take reference varaible.
///
/// This function doesnt do anything interesting, we pass a 'static
/// str and we get the same static string as a result. This will only
/// print the value when calling the function
/// ```
/// use pointers_threads::lib_th1a::*;
/// use std::thread;
///
/// let val:&'static str = "hello world!";
/// assert_eq!(val, "hello world!");
///
/// ```
pub fn thread1a_stat_owned(val: &'static str) -> &'static str {
    // This is quite, interesting. Even if we use 'static for the value
    // that is passed as an arguement, we still would have to use move
    // when using the closure. The reason is despite the closure needing
    // 'static for lifetime arguements, here val is a variable name that
    // is only valid for the lifetime of thread1a_stat_owned. We need to
    // remember that val itself is a reference that is of the COPY type.
    // Meaning, when we pass a value to thread1a_stat_owned, as its of
    // type &, this function copied the reference of the value. For this
    // reason, by using move, we only pass a reference variable into the
    // closure. As this would not be able to modify the type.
    let th = thread::spawn( move || {
        println!("thread1a stat val is {val}");
        val
    });
    th.join().unwrap()
}


//     th1::th1a::thread1a_scope();
// todomanish. this somewhat works. but I need to add a recursion limit in this
// somehow
// #![recursion_limit = "256"]
pub fn thread1a_scope_vec<T>(rng: std::ops::Range<T>, printable: bool) -> Vec<T>
where T: Sized + std::fmt::Display + Send,
      std::ops::Range<T>: std::iter::Iterator
{
    #[allow(warnings)]
    let mut v2 = ('a'..'z').collect::<Vec<char>>();
    // let v = rng.collect::<Vec<T>>();
    let mut v = rng.collect::<Vec<T>>();
    // scope doesnt need to have reference of 'static lifetime.
    // But what is important to know is that we cant have more
    // than 1 mutable referece instance in the scope, but serveral shared references.
    // lifetimes can be smaller than static as scope finished the tread
    // by join at the end
    thread::scope(|s| {
        s.spawn(|| {
            if printable {
                for x in &v {
                    print!("{}.", x);
                }
                println!();
            }
        }); // these threads are run concurrently
        s.spawn(|| {
            if printable {
                for x in &v {
                    print!("{}-", x);
                }
                println!();
            }
        });
        // we will get the issue here, v cannot be borrowed as mutable, as it was already borrowed
        // as immutable. This is when we have spawned threads in scope, that have already captured
        // the value as shared reference, but then we are using this thread and the closure infers
        // this as mutable reference when we push a value to it. For this reason, rust doesnt allow
        // this. In this example, it simply mention 3, but its for types T depending on the function
        // signature.
        // s.spawn(|| {
        //     v.push(3);
        // }); // this is not possible

        // This this situation, rust borrows a mutable refernce for v2, but its only used in the
        // scope just 1se, which doesnt cause conflicts as no other new threads have been asking 
        // for shared/mutable refernce of v2.
        s.spawn(|| {
            v2.push('z'); // automatic borrow ( as mut ) occures here.
        });

        // creating this below, would then end up causing conflicts.
        // s.spawn(|| {
        //     v2.push('z'); // automatic borrow ( as mut ) occures here.
        // }); // cannot be possible as we already called 1st mut ref of v2
    }); // all the threads are joined here.
    v
}
