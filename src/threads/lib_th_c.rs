/*
//! This module tell us how we need to be await when we are using threads.
//! If we are not careful, we risk the potential of making the threads
//! hang forever, as our threads could end up waiting for a `lock` to release
//! that might never happen. See [`std::sync::Mutex::lock`]. Consider the 
//! following example
//! ```ignore
//! let x = std::sync::Mutex::new(10);
//! {
//!     let y = x.lock().unwrap();
//!     // y is not dropped. and lifetimes for y ends only that the end of
//!     // scope. z is not stuck forever waiting to acquire the lock. while
//!     // y lock is yet not dropped
//!     // let z = x.lock().unwrap();
//! }
//! println!("finished");
//! ``` 
//! This is classic example of what might happen, and we have to be sure that these
//! types of implementation dont happen, as we will encounter this problem.
//!
//! Consider this way instead
//! ```
//! let m = std::sync::Arc::new(std::sync::Mutex::new(100u8));
//! let m_clone = m.clone();
//!
//! // For standalone spawned thread, we will have to pass the value via arc, because
//! // of if we plan on using more than one threads.
//! let th1 = std::thread::spawn(move || for _ in 0..20{
//!     let mut guard = m_clone.lock().unwrap();
//!     *guard = guard.saturating_add(1);
//! });
//!
//! let m_clone = m.clone();
//! let th2 = std::thread::spawn(move || for _ in 0..20{
//!     let mut guard = m_clone.lock().unwrap();
//!     *guard = guard.saturating_add(1);
//! });
//!
//!
//! let c_clone = m.clone();
//! let th3 = std::thread::spawn(move || for _ in 0..20{
//!     let mut guard = c_clone.lock().unwrap();
//!     *guard = guard.saturating_add(1);
//! });
//!
//! th1.join().unwrap();
//! th2.join().unwrap();
//! th3.join().unwrap();
//!
//! assert_eq!( 100+20+20+20, *m.lock().unwrap() );
//! ```
//
// let mx = std::sync::Mutex::new(val);
// let mut v: Vec<u16> = vec![];
// let mxv = std::sync::Mutex::new(vec![1]);
//
// std::thread::scope(|s| {
//     s.spawn(|| for _ in 1..=20 {
//         let mut guard = mx.lock().unwrap();
//         // but this is dangerous... 2 locks at the same time without dropping 1st one.
//         // Also lifetime for mx and guard needs to be observed as it doesnt necessarily 
//         // mean that it will end at the last call. I could end at the end of the scope.
//         // let mut guard = mx.lock().unwrap();
//
//         *guard = guard.saturating_add(1);
//         v.push(*guard); 
//         // this works but can un unsafe as ordering will cause issues
//         // if not accounted for. eg, we may push 23 or 24 depending on
//         // which thread runs 1st, if val is 22.
//         let mut guard2 = mxv.lock().unwrap();
//         guard2.push(*guard);
//     });
//     s.spawn(|| for _ in 1..=20 {
//         let mut guard = mx.lock().unwrap();
//         *guard = guard.saturating_add(1);
//
//         // via captured variables it doesnt work, but through the mutex pointer
//         // it does. The reasone it doesnt work is because we already have borrowed
//         // the value in the 1st thread, and we cant borrow it again. This is the 
//         // reason when we prefer to use it via Mutexes.
//         // v.push(*guard); // this doesnt work
//
//         let mut guard2 = mxv.lock().unwrap();
//         guard2.push(*guard);
//     });
// });
//
// let mut temp = mx.lock().unwrap();
// *temp = temp.saturating_add(100);
// // Dont forget to drop the temp value.
// drop(temp);
//
*/
use std::collections::vec_deque::VecDeque;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use rayon::prelude::*;

/// This function will hang forever when using mutexes this way.
/// 
/// This is just an illustration for [`std::sync::Mutex`] that, when we dont receive the lock
/// from `Lock()`, doesnt mean that the thread will panic, instead the thread is put to sleep
/// while waiting for the lock to be released. Here, the lock is never released before the
/// other lock() is called, and hence this will be hung forever and called be passed on.
/// ```ignore
/// # use pointers_threads::lib_th_c::*;
/// assert_eq!( 5u8, thread1c_mutex_hangs_forever(5, true) );
/// ```
pub fn thread1c_mutex_hangs_forever(val: u8, printable: bool) -> u8 {
    let toprint = |s:&str| { 
        if printable { println!("{}", s) }
    }; 

    let x = std::sync::Mutex::new(val);
    toprint("Before calling double lock");
    {
        #[allow(unused)]
        let y = x.lock().unwrap();
        toprint("first lock received");
        #[allow(unused)]
        let z = x.lock().unwrap();
    }
    
    // We will not see this statement printed.
    toprint("It doesn't panic, it just get stuck (deadlock) if a second lock is called. \
        Dead lock tried to get the lock even it knows that it cant as the 1st lock is not \
        dropped before using the 2nd.");
    
    // This will not be possible
    *x.lock().unwrap()
}


/// A simple function to illustrate how we have to use thread locking. This function just
/// give us back an increment of plus 200 of the value we passed if it doesnt overflow. Otherwise,
/// It will give us the max u16 value.
/// 
/// We call the 'Lock' method on the 'Mutex' to get the lockguard for the threads called
/// inside this fucntion. Here, we can run this as printable to understand the workflow of
/// how the threads try to get the lock. Its quite possible that the lock is already been
/// acquired by another threads, and hence the thread seeking the lock will have to wait 
/// till it gets free. Remember, that the thread will not panic in this instance.
/// ```
/// # use pointers_threads::lib_th_c::*;
///
/// let val = 5u16;
/// assert_eq!( val + 200, thread1c_mutex_lock_attempt(true, val) );
///
/// let val = 65500u16;
/// assert_eq!( u16::MAX, thread1c_mutex_lock_attempt(true, val) );
/// ```
pub fn thread1c_mutex_lock_attempt(printable: bool, val: u16) -> u16 {
    let toprint = |s:&str| {
        if printable { print!("{}", s) }
    };

    toprint("simple test for references\n");
    let mx = std::sync::Mutex::new(val);
    let mut v: Vec<u16> = vec![];
    let mxv = std::sync::Mutex::new(vec![1]);

    // Note for scope, we dont have to pass the value via arc, becaues scope takes a 
    // referenct for the mutex. This way, we know that lifetimes are respected and will
    // not forget to join before main thread is finished.
    std::thread::scope(|s| {
        s.spawn(|| for _ in 1..=20 {
            let mut guard = mx.lock().unwrap();
            // but this is dangerous... 2 locks at the same time without dropping 1st one.
            // Also lifetime for mx and guard needs to be observed as it doesnt necessarily 
            // mean that it will end at the last call. I could end at the end of the scope.
            // let mut guard = mx.lock().unwrap();

            *guard = guard.saturating_add(1);
            v.push(*guard); 
            // this works but can un unsafe as ordering will cause issues
            // if not accounted for. eg, we may push 23 or 24 depending on
            // which thread runs 1st, if val is 22.
            let mut guard2 = mxv.lock().unwrap();
            guard2.push(*guard);
        });
        s.spawn(|| for _ in 1..=20 {
            let mut guard = mx.lock().unwrap();
            *guard = guard.saturating_add(1);

            // via captured variables it doesnt work, but through the mutex pointer
            // it does. The reasone it doesnt work is because we already have borrowed
            // the value in the 1st thread, and we cant borrow it again. This is the 
            // reason when we prefer to use it via Mutexes.
            // v.push(*guard); // this doesnt work

            let mut guard2 = mxv.lock().unwrap();
            guard2.push(*guard);
        });
    });

    toprint( &format!("val for mx is now :{:?}.\n", mx) );
    toprint( &format!("val for vec is now :{:?}\n", mxv) );

    let mut temp = mx.lock().unwrap();
    *temp = temp.saturating_add(100);
    // Dont forget to drop the temp value.
    drop(temp);

    let m = Arc::new(mx);
    let m1 = m.clone();

    // For standalone spawned thread, we will have to pass the value via arc, because
    // if plan on using more than one threads.
    let th1 = thread::spawn(move || for _ in 0..20{
        let mut guard = m1.lock().unwrap();
        // but this is also dangerous... 2 locks at the same time without dropping
        // the 1st one, can lead to thread blocking permanently.
        // let mut guard = mx.lock().unwrap();

        *guard = guard.saturating_add(1);
    });


    let m1 = m.clone();
    let th2 = thread::spawn(move || for _ in 0..20{
        let mut guard = m1.lock().unwrap();
        *guard = guard.saturating_add(1);
    });


    let m2 = m.clone();
    let th3 = thread::spawn(move || for _ in 0..20{
        let mut guard = m2.lock().unwrap();
        *guard = guard.saturating_add(1);
    });

    th1.join().unwrap();
    th2.join().unwrap();
    th3.join().unwrap();

    toprint( &format!("val is now total :{:?}\n", m) );
    toprint( &format!("val for vec elements :{:?}\n", m) );
    *m.lock().unwrap()
}


/// This function is a bit more complex the the different operations it can do. However, 
/// essentially it tries to add 100 to the value that is passed to it. 
///
/// We do have the option to run this via Rayon, but we can see the complications for this
/// when we do so. As we use parking and unparking operations for this thread, we observe
/// that the rayon make it more complicated for thread parking, and generally its intended
/// to not use rayon for such operations.
/// 
/// We also have the option to run it normally without rayon, which make our function 
/// run as expected. This function allus us to also print some details when we execte this
/// function. This function encomposes mutexes, locks, parking, unparking and rayon, but
/// its good to get a hang of how this works. 
///
/// # Warning
///
/// This test should pass, we dont use rayon here. However, it will definitely depend on the 
/// timing values sent, otherwise we not not be able to get the results we want
/// ```
/// # use pointers_threads::lib_th_c::*;
/// let val = 5u8;
/// // For this implementation, notice we dont use rayon here.
/// let val = thread1c_mutex_lock_attempt_inc_drop( std::sync::Mutex::new(val), true, false, true, 40 );
///
/// assert_eq!(105u8, *val.lock().unwrap() );
/// ```
/// This has a good change of panicking, if we reduce the time, and increase the value
/// even if value if small, we can see issues, but will be more definate when we increase the
/// size. 
/// ```should_panic
/// # use pointers_threads::lib_th_c::*;
/// let val = 5u8;
/// // For this implementation, notice we dont use rayon here.
/// let val = thread1c_mutex_lock_attempt_inc_drop( std::sync::Mutex::new(val), true, false, true, 0 );
///
/// assert_eq!(105u8, *val.lock().unwrap() );
/// ```
/// # Warning
///
/// This function has a high potential of failing. However, there are changes where it could also
/// pass. This is because we will be using rayon here. Just tries to run the park and unpark 
/// synchronization parallelly. This implementation is incorrect, and should not be used.
///
/// Rayon should be handled carefully when we work with threads/tasks in general.
/// ```
/// # use pointers_threads::lib_th_c::*;
/// let val = 5u8;
/// // For this implementation, we use rayon
/// let val = thread1c_mutex_lock_attempt_inc_drop( std::sync::Mutex::new(val), true, true, true, 20 );
///
/// // Notice here, I use the greater than and equal-to check rahter than just equal-to 
/// // check, because there is a very good chance of this implmentation, not being
/// // able to output an value that is an increment of the arguement passed.
/// assert!(105u8 >= *val.lock().unwrap() );
///
/// // This could very likely fail as loop increment size increases.
/// // assert!(105u8 > *val.lock().unwrap() );
/// ```
///
/// # Warning
///
/// If we were to provide the false value for the drop arguement, we will notice that
/// this will hang forever, and this goes to show that we will have to always drop the value
/// if were to ask for another lock within the same scope. There should never to more than
/// one lock statment if there already exists a mutexguard for the variable in question.
///
/// ```ignore
/// # use pointers_threads::lib_th_c::*;
/// let val = 5u8;
/// // For this implementation, we pass drop to false, causing it to hang forever.
/// let val = thread1c_mutex_lock_attempt_inc_drop( std::sync::Mutex::new(val), false, true, true, 10 );
///
/// assert_eq!(105u8, *val.lock().unwrap() );
/// ```
pub fn thread1c_mutex_lock_attempt_inc_drop(
    val: Mutex<u8>, is_dropable: bool, is_rayon: bool,
    printable: bool, time_ms: u64
) -> Mutex<u8>
{
    let toprint = |s:&str| { 
        if printable { println!("{}", s) }
    }; 

    let finish = Mutex::new(false);
    // this is just to illustrate that mutex lock, when not received for Lock(), doesnt mean that
    // the tread will panic, instead the thread is put to sleep while waiting for the lock to
    // be released. Here, the lock is never released before the other lock() is called, and hence
    // this will be hung forever and called be passed on.
    thread::scope(|s| {
        let th1 = s.spawn(||loop {
            toprint("Before calling double lock");
            {
                // Note: Mutex Guard arent using a lifetime here in the sence that
                // the check, doesnt drop it when we dont use y. This mean that we
                // will manually have to drop it, before another lock is called for 
                // that variable, of it would hang that thread.
                let y = val.lock().unwrap();
                toprint( &format!("first lock received {}", y) );
                if is_dropable {
                    toprint("lock is dropped");
                    // if this drop was not here, this program will hang forever
                    drop(y);
                } else {
                    toprint("lock not dropped");
                }
                #[allow(unused)]
                let z = val.lock().unwrap();
                toprint( &format!("second lock received {}", z) );
                // drop(z); // This drop will happen automatically
            }
            toprint("Notice: It doesn't panic, lock can block thread if a second lock \
                is called at the same time, for that variable.\n");
            std::thread::park(); // we manually park this thread casee we loop it.
            toprint( "th1 got unparked");
            // Note: this could panic potentially panic in production, but for simplicity
            // we use unwrap for showing how to work with locks.
            if *finish.lock().unwrap() { break }
        });

        //NOTE: what if we use rayon here.
        //Since, rayon can create multiple processes at the same time here,
        //the issues when using mutexes and locks, if not properly configured,
        //could result in errors. A common error is how we park and unpark will
        //be used during parallel calls. The unpark call happens in parallel, 
        //causing the main thread to complete, without without any account taken
        //if the th1 thread is still parked or not.
        //In this situation, we can see that it causes issues and not what we
        //wanted.
        let cal = |i: u8| {
            //NOTE: This timing value if not correctly configured, will cause the
            //function to fail
            thread::sleep(std::time::Duration::from_millis( time_ms ));
            toprint( &format!("trying to unpark/wakeup spawned thread. Loop iteration {}", i) );
            th1.thread().unpark();
                toprint( "main got it: main");
            if let Ok(mut v) = val.try_lock() {
                toprint( &format!("got the lock for loop {}", i) );
                *v = v.saturating_add(1)
            } else {
                toprint( &format!("didnt get lock for loop {}", i) );
            }
        };
        if is_rayon {
            (1..=100u8).into_par_iter().for_each( |i| {
                cal(i);
            });
        } else {
            for i in 1..=100u8 {
                cal(i);
            }
        }

        toprint( "Loop finished, if you dont see any print statements after this, thread failed");
        //NOTE: Since we could use Rayon, there is a very high change that the
        //main thread doesnt unpark the th1 thread. So we manually call 
        //unpark here.
        th1.thread().unpark();
        *finish.lock().unwrap() = true;
    });
    toprint("However, thread passed\n"); // this is not possible
    val
}


/// testing park_mut
/// ```
/// # use pointers_threads::lib_th_c::*;
///
/// // We will have to get a vec of 100 elements, that should be in theory
/// // arranged in the correct order
/// let val = thread1c_park_mutex_create_vec_size( 100, true, 0 );
///
/// assert_eq!(
///     val.unwrap().into_iter().map(|v| v as u16).sum::<u16>(),
///     (1..=100u16).into_iter().sum::<u16>()
/// );
/// ```
/// The inner loop will park threads like this.
/// ```
/// let queue = std::sync::Mutex::new( std::collections::VecDeque::new());
/// std::thread::scope(|s| {
///     let t1 = s.spawn(|| loop {
///         let value = queue.lock().unwrap().pop_back();
///         match value {
///             Some(Some(val)) => {
///                 // do something with val
///             }
///             Some(None) => break,
///             None => std::thread::park(),
///         }
///     });
///     for x in 1..=10 {
///         queue.lock().unwrap().push_front(Some(x));
///         t1.thread().unpark();
///         std::thread::sleep(std::time::Duration::from_millis( 80 ));
///         if x == 10 {
///             queue.lock().unwrap().push_front(None);
///             t1.thread().unpark();
///         }
///     }
/// })
/// ```
pub fn thread1c_park_mutex_create_vec_size(val: u8, printable: bool, milli_sec: u64) -> Option<Vec<u8>> {
    let toprint = |s:&str| { 
        if printable { println!("{}", s) }
    }; 
    toprint("---------------------------thread park for mutex-----------------------");
    // for 0 values, we send expty vector
    if val == u8::MIN { return Some(vec![]) };

    // We dont need this, but we are just playing around with the code to see how we can
    // convert this type. And as we are using thread scope, I dont need to worry about
    // putting it into a mutex.
    let mut v = VecDeque::<Option<u8>>::new();

    // Technically, we should be looking at having a huge capicity, as elements should
    // be popped off in tandom.
    let queue = Mutex::new(VecDeque::<Option<u8>>::with_capacity(10));
    thread::scope(|s| {
        // loop is called we have to get all the values from the vector
        let t1 = s.spawn(|| {
            loop {
                // Works, but this will be infinite loop. So I will break it up so that
                // we accommodate for break in the loop. This was originally for VecDeque u8. Not
                // VecDeque option u8
                // let guard = queue.lock().unwrap().pop_back();
                // 
                // // the guard lock is not used after this. I imagine the compiler is able to hand
                // // over the lock to a different thread if needed from this point onwards
                // if let Some(g) = guard {
                //     dbg!(g);
                // } else {
                //     // just by calling thread park, we is able to choose this tread to park.
                //     // blocks thread and doesnt use the thread anymore tell it gets unparked
                //     thread::park();
                // }

                // NOTE: If I use queue.lock().unwrap().pop_back() with match directly like that,
                // I will get an issue. The lock will not be released till the match scope ends.
                // The mutex will be bound to the life of the match scope. and hence will not
                // get to drop earlier. eg match queue.lock().unwrap().pop_back() { ... }
                // This is not what we want. We want the thread to be availeble immediately after we
                // call the pop_back() value. And we do this be splitting the guard from the match
                // statement. So the below statement is not recommented
                // match queue.lock().unwrap().pop_back() {
                // Hence: it is better to break it up like the way we have it below
                // In Polonius, this will change, and should be able to handle the lifetime check
                // without getting any problems.
                let value = queue.lock().unwrap().pop_back();
                // mutexguard already discarded after this cause no variable directly holds it
                match value {
                    Some(Some(val)) => {
                        toprint( &format!("{}", val ) );
                        v.push_front( Some(val) );
                    }
                    Some(None) => break,
                    None => thread::park(),
                }
            }
        });

        for x in 1..=val {
            // Mutexguard is released immediately as its not held in a variable
            // If we added this in a variable, even with lifetimes, this would 
            // cause problems
            queue.lock().unwrap().push_front(Some(x));
            // By unparking this thread, it will be able to trigger the thread to wake up
            // so that is will print all the items it needed
            t1.thread().unpark();
            // having sleep or not should not affect the thread in this scenerio
            // We sleep hoping the park has waken up the t1 thread
            thread::sleep(Duration::from_millis( milli_sec ));

            // We arrive at the end of our call
            if x == val {
                queue.lock().unwrap().push_front(None);
                // its important to call thread unpark or else the thread could be stuck in park if
                // the value is not handled for Some of None.
                t1.thread().unpark();
            }
        }

        // this wont work as it only drops the thread handle if we need to stop the thread
        // drop(t1);
        println!(" we have final for queue {:?}", queue.lock().unwrap());
    });

    // this runs in O(1), as there is no reallocation from vec to vecdeque. 
    // but we will have to accocate for vec in this case.
    // Again, we dont need to do this, its quite useless, but it a good way
    // to get familiar with this.
    // std::collections::VecDeque::from(vec![1,2,3,4,5]);
    // let x: Vec<i32> = std::collections::VecDeque::from([1,2,3,4,5]).into();
    
    // We also should not be getting and None elements
    // Some( v.iter().map(|val| val.unwrap_or(0)).collect::<Vec<u8>>() )
    // Safety:
    // We always pass only Some(val) type in the creation of v, so we should
    // never get None.
    Some( v.iter().map(|val| unsafe { val.unwrap_unchecked() } ).collect::<Vec<u8>>() )
}


// Just a simple struct to show how values look for mutexes
#[allow(unused)]
#[derive(Debug)]
pub struct TestMutexArc<'a> {
    pub a: std::sync::Mutex<u8>,
    pub b: std::sync::Mutex<u16>,
    pub c: std::sync::Mutex<&'a str>,
}

/// Function used to see the states for a mutex and how they are diplayed.
/// 
/// This will allows to have a peak of how this works and what we are to 
/// expect when certain values are locked or now. Simple approach to understand 
/// when we are working with Mutexes
///
/// This funcion has come inner functions, and we can call them separately to see
/// how they work.
/// From 
/// ```
/// pub struct TestMutexArc<'a> {
///     pub a: std::sync::Mutex<u8>,
///     pub b: std::sync::Mutex<u16>,
///     pub c: std::sync::Mutex<&'a str>,
/// }
/// ```
/// We compute the following
/// ```
/// # use pointers_threads::lib_th_c::*;
/// let x = std::sync::Mutex::new(33);
/// let y = std::sync::Mutex::new(88);
/// let z = std::sync::Mutex::new("Hello there");
///
/// // x,y,z are all moved to the tmutx here
/// let tmutx = TestMutexArc { a: x, b: y, c: z };
///
/// // this will give us a locked value of data
/// // _x will still hold the mutex guard
/// let _x = tmutx.a.lock();
/// assert!( tmutx.a.try_lock().is_err() );
/// ```
/// We could also do something like this
/// ```
/// let a = std::sync::Arc::new(33);
/// let b = std::sync::Arc::clone(&a);
/// let c = std::sync::Arc::clone(&a);
///
/// // We could use scope thread here to make it more simpler.
/// std::thread::spawn(move || {
///     println!("b is moved to this spawned thread: {}", b);
/// }).join().unwrap();
///
/// std::thread::spawn(move || {
///     println!("c is moved to this spawned thread: {}", c);
/// }).join().unwrap();
///
/// // we can omit creating var like b,c by calling it inside the scope to make it cleaner
/// std::thread::spawn({
///     let a = a.clone();
///     move || {
///        println!("a is moved to this spawned thread: {}", a);
///     }
/// });
/// ```
/// We could also use rayon in threads/closures with our arc looping. This will
/// simple demonstrate how we can use closures and pass them to threads scope and
/// mutate a value of an arc by acquiring the mutexguard.
/// Have a look at [`rayon`] and its lib.
/// ```
/// use rayon::prelude::*;
/// let x = std::sync::Arc::new(std::sync::Mutex::new(33u64));
/// let y = std::sync::Arc::clone(&x);
/// let func = || {
///     // y is passed here as reference, not moved cause closures in itself 
///     // dont need to have static reference lifetimes. This is only a requirement
///     // on thread spawn. If we however call thread spawn inside of a thread scope
///     // object, it is able to use this reference as it seen in func and func2
///
///     // if let Ok(ref mut guard) = y.lock()
///     // this works too and we take **guard+=x;
///     if let Ok(mut guard) = y.lock() {
///         // NOTE: it not okay to use rayon like this. This is because we will
///         // have to pass arc and not MutexGuard, as rayon will complain that we can
///         // use Send for MutexGuard.
///         for x in 1..=1000 {
///             *guard += x;
///         }
///     } 
/// };
/// let func2 = || {
///     if let Ok(mut guard) = y.lock() {
///         for x in 1..=100_000 {
///             *guard += x;
///         }
///     }
/// };
/// // if we use rayon, we have to pass in here by calling arc in our instance.
/// // This implementation is not the best, as arc is expensive, but it servers
/// // our purpose to show how we could use rayon here.
/// let func2_rayon = || {
///     (1..=100_000).into_par_iter().for_each( |x| {
///         // I have to clone it as we pass many arc values for the parallel iterator
///         let y = y.clone();
///         if let Ok(mut guard) = y.lock() {
///             *guard += x;
///         }
///     });
/// };
/// std::thread::scope( |s| {
///     s.spawn(func);
///     s.spawn(func2);
///     s.spawn(func2_rayon);
///     for x in 1..=10 {
///         // 10 threads are spawned here. So its all okay to testing
///         let n = y.clone();
///         s.spawn( move || loop { 
///         // move is required here, as closure might outlive the arc value
///
///             // try lock will not block it tests if it gets the lock
///             // but we will have to keep attempting to get the lock which is why we put
///             // it in a loop
///             if let Ok(mut guard) = n.try_lock() {
///                 for y in 1..=20 {
///                     *guard += 1;
///                 }
///                 break;
///             }
///             else {
///                 // we didnt get the lock, so we sleep the thread a bit.
///                 std::thread::sleep(std::time::Duration::from_millis(50));
///             }
///         });
///     }
/// }); // all thread join here
///
/// // Here, we take Arc -> into_inner => mutex. and into_inner whick also works
/// assert_eq!( unsafe { *x.lock().unwrap_unchecked() },
///     33+(1..=1000).sum::<u64>()+( (1..=100_000).sum::<u64>()*2 )+ 200 );
/// ```
/// This function simply runs in this  way.
/// ```
/// // when we print true, it gets imputted to stdout
/// // this produces a vec<string> of the log for the function
/// # use pointers_threads::lib_th_c::*;
/// let _ = thread1c_arc_mutex_display( &["display", "move"], false);
/// let _ = thread1c_arc_mutex_display( &["loop"], false);
///
/// // success would if the function doesnt panic
/// ````
pub fn thread1c_arc_mutex_display( input: &[&str], printable: bool) -> Vec<String> {
    // NOTE: deliverately are using &mut vec<string> as arguement,
    // I would be easier to use toprint function be just add the vector elements,
    // however it fun to push the limits of this implementation, and try to pass
    // it as a reference. In order to do this. We use static functions.

    static mut PRINT_VAL_ARC_DISPLAY: bool = false;
    unsafe { PRINT_VAL_ARC_DISPLAY = printable; }

    let toprint = move |s:&str| {
        if unsafe { PRINT_VAL_ARC_DISPLAY } { println!("[print]: {s}"); }
    };

    let mut vec_string: Vec<String> = vec![];

    fn lock_display( print: fn(&str), vec_string: &mut Vec<String> ) {
        vec_string.push("\n-------------------------Lock display------------------------------".to_string() );
        print( unsafe { vec_string.last().unwrap_unchecked() });

        let x = std::sync::Mutex::new(33);
        let y = std::sync::Mutex::new(88);
        let z = std::sync::Mutex::new("Hello there");

        // x,y,z are all moved to the tmutx here
        let tmutx = TestMutexArc { a: x, b: y, c: z };


        vec_string.push( format!( "Struct TestMutexArc before: {:#?}\n", tmutx) );
        print( unsafe { vec_string.last().unwrap_unchecked() });

        // this will give us a locked value of data
        let _x = tmutx.a.lock();
        vec_string.push( format!("Struct TestMutexArc after: {:#?}\n", tmutx) );
        print( unsafe { vec_string.last().unwrap_unchecked() });
    } // drop(_x); called automatically


    fn arc_move( print: fn(&str), vec_mut: &mut Vec<String> ) {
        let mut vec_string: Vec<String> = vec![];
        vec_string.push("\n-------------------------Arc move------------------------------".to_string() );
        print( unsafe { vec_string.last().unwrap_unchecked() });

        let a = std::sync::Arc::new(33);
        let b = std::sync::Arc::clone(&a);
        let c = std::sync::Arc::clone(&a);

        vec_string.push( format!("Creating Arcs: \na:{a:?}\nb:{b:?}\nc:{c:?}\n" ) );
        print( unsafe { vec_string.last().unwrap_unchecked() });

        //NOTE: We could add scope threads here to avoid, but it wanted to use
        //spawn to see how it would look
        let mut vec_string = std::thread::spawn(move || {
            vec_string.push( format!("For b that is moved in this pointer arc is: {}", b) );
            print( unsafe { vec_string.last().unwrap_unchecked() });
            vec_string
        })
        .join()
        .unwrap();

        let mut vec_string = std::thread::spawn(move || {
            vec_string.push( format!("For c is also moved in this arc is: {}", c) );
            print( unsafe { vec_string.last().unwrap_unchecked() });
            vec_string
        })
        .join()
        .unwrap();

        // we can omit creating var like b,c by calling it inside the scope to make it cleaner
        let jn = std::thread::spawn({
            let a = a.clone();
            move || {
                vec_string.push( format!("New a arc is: {}", a) );
                print( unsafe { vec_string.last().unwrap_unchecked() });
                vec_string
            }
        });

        vec_mut.append(&mut jn.join().unwrap() );
    } // a is dropped here, even if use clone


    fn arc_looping( print: fn(&str), vec_string: &mut Vec<String> ) {
        use rayon::prelude::*;

        // I dont need to define a new vec_string
        // let mut vec_string: Vec<String> = vec![];
        vec_string.push("\n-------------------------Arc Looping------------------------------".to_string() );
        print( unsafe { vec_string.last().unwrap_unchecked() });

        let x = std::sync::Arc::new(std::sync::Mutex::new(33u64));
        let y = std::sync::Arc::clone(&x);
        let z = std::sync::Mutex::new(vec![1, 2, 3, 4, 5]);

        let func = || {
            let mut vec_clone: Vec<String> = vec![];
            // y is passed here as reference, not moved
            // cause closures in itself dont need to have static
            // reference lifetimes. This is only a requireemnt
            // on thread spawn. If we however call thread spawn
            // inside of a thread scope object, it is able to use
            // this reference as it seen in func and func2
            // if let Ok(ref mut guard) = y.lock() { // this works too and we take **guard+=x;
            if let Ok(mut guard) = y.lock() {
                // for x in 1..=1_000_000_000 {
                // NOTE: it not okay to use rayon like this. This is because we will
                // have to pass arc and not MutexGuard, as rayon will complain that we can
                // use Send for MutexGuard.
                for x in 1..=1000 {
                    *guard += x;
                }
                vec_clone.push( format!("Fun1: Mutex mutated in \
                    spawned thread for y is {}", guard) );
                print( unsafe { vec_clone.last().unwrap_unchecked() });
            } else {
                vec_clone.push( "Fun1 else: didnt get the lock for y".to_string() );
                print( unsafe { vec_clone.last().unwrap_unchecked() });
            }
            vec_clone
        };

        let func2 = || {
            let mut vec_clone: Vec<String> = vec![];
            // since y was taken as reference before for func,
            // y can be taken as ref again as y was dropped before I imagine
            // if let Ok(ref mut guard) = y.lock() { // also works
            if let Ok(mut guard) = y.lock() {
                // for x in 1..=1_000_000_000 {
                for x in 1..=1_000_000 {
                    *guard += x;
                }
                vec_clone.push( format!("Fun2: mutex mutated in \
                    spawned thread for new y is {}", *guard) );
                print( unsafe { vec_clone.last().unwrap_unchecked() });
            } else {
                vec_clone.push( "Fun2 else: didnt get the lock for y new".to_string() );
                print( unsafe { vec_clone.last().unwrap_unchecked() });
            }
            vec_clone
        };

        let func2_rayon = || {
            let vec_clone_arc: std::sync::Arc<std::sync::Mutex<Vec<String>>> = 
                std::sync::Arc::new(std::sync::Mutex::new(vec![]));
            (1..=1_000_000).into_par_iter().for_each( |x| {
                let y = y.clone();
                let vec_c = vec_clone_arc.clone();
                if let Ok(mut guard) = y.lock() {
                    *guard += x;
                    if x == 1_000_000 {
                        vec_c.lock().unwrap().push(format!("Fun2 Rayon: mutex mutated in \
                            spawned thread for new y is {}", *guard) );
                        let cloned_vec = vec_c.lock().unwrap();
                        print( unsafe { (cloned_vec).last().unwrap_unchecked() });
                    }
                } else {
                    vec_c.lock().unwrap().push(format!("Fun2 Rayon else: didnt get the lock for y \
                        new 1_000_000 for {} loop", x) );
                    let cloned_vec = vec_c.lock().unwrap();
                    print( unsafe { (cloned_vec).last().unwrap_unchecked() });
                }
            });
            match vec_clone_arc.lock() {
                Ok(val) => val.to_vec(),
                // we use into inner here to get the value even if its poisoned
                // I dont need to worry about clearing poison here.
                Err(poisoned) => poisoned.into_inner().to_vec(),
            }
        };

        // this will not work directly, as spawn needs static lifetime.
        // we will have to use a scope thread that has spawn inside it.
        // thread::spawn(func);

        let func3 = || {
            let mut vec_clone: Vec<String> = vec![];
            loop {
                // can be same a lock, except that this will not hang for try_lock if 
                // lock not recieved 
                // if let Ok(ref mut vec) = z.try_lock() {  // works
                if let Ok(mut vec) = z.try_lock() {
                    vec_clone.push( "Fun3: lock received for z".to_string() );
                    print( unsafe { vec_clone.last().unwrap_unchecked() });
                    if let Some(val) = vec.get_mut(4) {
                        *val += 10;
                    }
                    vec_clone.push( "Fun3: lock received for z".to_string() );
                    print( unsafe { vec_clone.last().unwrap_unchecked() });
                    println!("vec is {:?}", vec);
                    break;
                } else {
                    vec_clone.push("Fun3: lock not received".to_string());
                    print( unsafe { vec_clone.last().unwrap_unchecked() });
                    thread::sleep(std::time::Duration::from_millis(200));
                }
            }
            vec_clone
        };

        let m = std::sync::Mutex::new(0);
        let n = std::sync::Arc::new(m);

        let v = std::sync::Mutex::new(vec_string);
        let v = std::sync::Arc::new(v);

        thread::scope( |s| {
            let vec1 = s.spawn(func);
            let vec2 = s.spawn(func2);
            let vec3 = s.spawn(func2_rayon);
            let vec4 = s.spawn(func3);

            for x in 1..=10 {
                // 10 threads are spawned here. So its all okay to testing
                let n = std::sync::Arc::clone(&n);
                let vec_c = v.clone();
                // since x would have been taken as refernece for print statement, we have to use
                // move cause we cant be sure that it will life long enougth
                // The compiler cant verify even it this is possible.
                // And because we will have to end up using move, we have to pass our data in Arc clone
                s.spawn(move || {
                    loop {
                        // try lock will not block it tests if it gets the lock
                        // but we will have to keep attempting to get the lock which is why we put
                        // it in a loop
                        if let Ok(mut guard) = n.try_lock() {
                            for y in 1..=20 {
                                *guard += 1;
                                // to avoid too many print statements
                                if y % 5 == 0 {
                                    vec_c.lock().unwrap().push( format!("Main Loop {}: guard is now \
                                        {} at {}", x, *guard, y) );
                                    let v = vec_c.lock().unwrap();
                                    print( unsafe { v.last().unwrap_unchecked() });
                                }
                            }
                            break;
                        } else {
                            vec_c.lock().unwrap().push( format!("didnt get lock for {x}, trying \
                                after some milli seconds") );
                            let v = vec_c.lock().unwrap();
                            print( unsafe { v.last().unwrap_unchecked() });
                            thread::sleep(std::time::Duration::from_millis(50));
                        }
                    }
                });
            }

            let mut vec_c = v.lock().unwrap();
            vec_c.append( &mut vec1.join().unwrap() );
            vec_c.append( &mut vec2.join().unwrap() );
            vec_c.append( &mut vec3.join().unwrap() );
            vec_c.append( &mut vec4.join().unwrap() );

        }); // all thread join here
        // Here, we take Arc -> into_inner => mutex. and into_inner whick also works
        // assert_eq!(Arc::into_inner(n).unwrap().into_inner().unwrap(), 200);
        // lock should be dropped automatially after this scope
        assert_eq!(*n.lock().unwrap(), 200);

        print( &format!("{}, {}, {}. total should be {}, and what we got {}", 
            33, 
            (1..=1000).sum::<u64>(), 
            (1..=1_000_000).sum::<u64>(),
            33+(1..=1000).sum::<u64>()+( (1..=1_000_000).sum::<u64>()*2 ),
            unsafe { x.lock().unwrap_unchecked() }
        ));

        assert_eq!( unsafe { *x.lock().unwrap_unchecked() },
            33+(1..=1000).sum::<u64>()+( (1..=1_000_000).sum::<u64>()*2 ) );

    }


    for val in input.iter() {
        match *val {
            "display" => { lock_display( toprint, &mut vec_string ) },
            "move" => { arc_move( toprint, &mut vec_string ) },
            "loop" => { arc_looping( toprint, &mut vec_string ) },
            _ =>  {}
        }
    }

    vec_string
}


