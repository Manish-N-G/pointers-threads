// This is anohter addition of doc comments on top of what is 
// already mentioned before. //! is for lib_th_a again. And its
// the root of this crate. /// Will need to be appended only to
// another module under this or we will have to use it on functions
// which will show up the description in the doc file.
// Despite being able to do this, it still would be best to respect
// the way its handled in rust. We put comments to show how not to
// do it.

//! This module covers how different type of threads and pointers are used
//!
//! In order to understand threads, we will also have to worry about
//! pointers and what pointer is best to use. Hence all this is accessable
//! in thei module.
//!
//! These selected few functions is mainly used for thread creation
//! and errors that could be encountered when creating threads.
use std::thread;

// Remember, rust docs are not compiled inside lib_th_a
// but turns it into a separate crate.

/// Creates a spawned thread and Joins handle the returns a vec with 42.
///
/// This is a simple test thread that takes a vector, and adds the value
/// 42 to it at the end of the vec
/// ```
/// use pointers_threads::lib_th_a::*;
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
/// use pointers_threads::lib_th_a::*;
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
/// use pointers_threads::lib_th_a::*;
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
/// use pointers_threads::lib_th_a::*;
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
/// use pointers_threads::lib_th_a::*;
/// use std::thread;
///
/// assert_eq!(thread1a_stat(), "this is stat thread!");
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
/// use pointers_threads::lib_th_a::*;
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


/// We take 2 Iterator of a Generic type, and creates a shared reference thread
/// with one, and a mutable reference with another. It returns a vec of the
/// mutated iterator via a Vec.
///
/// This uses generics and a understand how threads process these types
/// of values when they are generic. This function doesnt do mut but
/// gives us an undstanding how to combine Generics with Iterators
/// and create threads
/// ```
/// use pointers_threads::lib_th_a::*;
/// use std::thread;
///
/// assert_eq!(
///     thread1a_scope_vec((1..10), ('a'..'z'), 'z', false),
///     ('a'..='z').collect::<Vec<char>>()
/// );
/// ```
/// There also consists of smaller sub threads in this function just to
/// get a feel for how the thread works. In order to understand it
/// better we need to implement the ptinr type.
/// ```
/// use pointers_threads::lib_th_a::*;
/// use std::thread;
///
/// // We can print the output when we run it as true.
/// // And these threads show how we can intermingle values.
/// // An example is already mentioned in the function. However,
/// // for duplicate unordered list, we can increase the value
/// // of the range to get a better outcome.
/// // note: this is handles by sleep internally to achived the
/// // ordering patter.
/// assert_eq!(
///     thread1a_scope_vec((1..1000), ('a'..'z'), 'z', true),
///     ('a'..='z').collect::<Vec<char>>()
/// );
/// ```
// With range, I was asked to sort out Recursion limits if we didnt use 
// Range: Iterator<item = type> but I didnt want to break my head, but instead,
// when I tried it, it didnt give me recursion problems.
// #![recursion_limit = "256"]
// I still prefer to use Iterator directly to avoid any hassle.
// pub fn thread1a_scope_vec<I1, I2, U1, U2>(iter1: std::ops::Range<U1>, iter2: std::ops::Range<U2>, val2: U2,  printable: bool)
//     -> Vec<U2>
// where std::ops::Range<U1>: Iterator<Item = U1> + Send + Sync,
//       std::ops::Range<U2>: Iterator<Item = U2> + Send + Sync,
//       Vec<U1>: std::fmt::Debug,
//       Vec<U2>: std::fmt::Debug,
//       U1: std::fmt::Debug + std::fmt::Display + Send + Sync,
//       U2: std::fmt::Debug + std::fmt::Display + Send + Sync,
pub fn thread1a_scope_vec<I1, I2>(iter1: I1, iter2: I2, val2: I2::Item,  printable: bool)
    -> Vec<I2::Item>
where I1::Item: std::fmt::Display + Send + Sync, // sized is default
      I2::Item: std::fmt::Display + Send + Sync,
      Vec<I2::Item>: std::fmt::Debug,
      I1: std::iter::Iterator,
      I2: std::iter::Iterator,
{
    let v = iter1.collect::<Vec<I1::Item>>();
    // let v = iter1.collect::<Vec<U1>>();
    #[allow(warnings)]
    let mut v2 = iter2.collect::<Vec<I2::Item>>();
    // let mut v2 = iter2.collect::<Vec<U2>>();
    // let v = rng.collect::<Vec<T>>();
    // scope doesnt need to have reference of 'static lifetime.
    // But what is important to know is that we cant have more
    // than 1 mutable referece instance in the scope, but serveral shared references.
    // lifetimes can be smaller than static as scope finished the tread
    // by join at the end
    let x: Vec<_> = (1..1000).collect();
    let y = std::sync::Arc::new( std::sync::Mutex::new(x.iter()));
    let z = y.clone();
    thread::scope(|s| {
        let s1 = s.spawn(|| {
            if printable {
                for x in &v {
                    print!("{}.", x);
                    thread::sleep(std::time::Duration::from_nanos(10));
                }
                println!();
            }
        }); // these threads are run concurrently
        let s2 = s.spawn(|| {
            if printable {
                for x in &v {
                    print!("{}-", x);
                    thread::sleep(std::time::Duration::from_nanos(10));
                }
                println!();
            }
        });
        // At this point, these both threads are running. Its just that when we call
        // join, we make sure that we dont proceed to the next command till join completes.
        // Here we make sure that s1 and s2 are complete before moving to the other thread.
        // However, s1 and s2 will be running at this position regardless if we called s1 join
        // over s2 join.
        // Not to forget, s1 and s2 both have its own reference so the values printed are
        // repeated, expect they threads print them simultaneously.
        // Note: adding a small thread sleep duration time can show better intermingling
        s1.join().unwrap();
        s2.join().unwrap();
        let t1 = s.spawn(|| {
            if printable {
                while let Some(val) = y.lock().unwrap().next() {
                    print!("{}**", val);
                    thread::sleep(std::time::Duration::from_nanos(10));
                }
                println!();
            }
        });
        let t2 = s.spawn(|| {
            if printable {
                while let Some(val) = z.lock().unwrap().next() {
                    print!("{}~~", val);
                    thread::sleep(std::time::Duration::from_nanos(10));
                }
                println!();
            }
        });
        // These two join will make sure that we have the same shared values and the print
        // will give us unique values but time to time handled by t1 or t2.
        t1.join().unwrap();
        t2.join().unwrap();
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
            v2.push(val2); // automatic borrow ( as mut ) occures here.
            if printable {
                println!("v2 is: {:?}", v2);
            }
        });

        // creating this below, would then end up causing conflicts.
        // s.spawn(|| {
        //     v2.push('z'); // automatic borrow ( as mut ) occures here.
        // }); // cannot be possible as we already called 1st mut ref of v2
    }); // all the threads are joined here.
    v2
}
