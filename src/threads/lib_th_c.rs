//todomanish: Have to make the lib for this.
// I can use the eample here to make sure that we are able to see hwo the
// threads area actually going to be pushed.
//! Here we start with the th c module.
//! ``` 
//! let x = 4u8;
//! ``` 
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
/// This test should pass, we dont use rayon here
/// ```
/// # use pointers_threads::lib_th_c::*;
/// let val = 5u8;
/// // For this implementation, notice we dont use rayon here.
/// let val = thread1c_mutex_lock_attempt_drop( std::sync::Mutex::new(val), true, false, true );
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
/// let val = thread1c_mutex_lock_attempt_drop( std::sync::Mutex::new(val), true, true, true );
///
/// // Notice here, I use the greater than and equal-to check rahter than just equal-to 
/// // check, because there is a very good chance of this implmentation, not being
/// // able to output an value that is an increment of the arguement passed.
/// assert!(105u8 >= *val.lock().unwrap() );
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
/// let val = thread1c_mutex_lock_attempt_drop( std::sync::Mutex::new(val), false, true, true );
///
/// assert_eq!(105u8, *val.lock().unwrap() );
/// ```
pub fn thread1c_mutex_lock_attempt_drop(
    val: Mutex<u8>, is_dropable: bool, is_rayon: bool, printable: bool) -> Mutex<u8>
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
            thread::sleep(std::time::Duration::from_millis(10));
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



pub fn thread1c_park_mutex() {

    println!("---------------------------thread park for mutex-----------------------");
    let queue = Mutex::new(VecDeque::<Option<u8>>::new());
    thread::scope(|s| {
        // loop is called we have to get all the values from the vector
        let t1 = s.spawn(|| {
            loop {
                // Works , but this will be infinite loop. So I will break it up so that
                // we accommodate for break in the loop. This was originally for VecDeque u8. Not
                // VecDeque option u8
                // let guard = queue.lock().unwrap().pop_back();
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
                let value = queue.lock().unwrap().pop_back();
                // mutexguard already discarded after this cause no variable directly holds it
                match value {
                    Some(Some(val)) => {
                        dbg!(val);
                    }
                    Some(None) => break,
                    None => thread::park(),
                }
            }
        });

        for x in 1..=10 {
            // mutexguard is released immediately as its not held in a variable
            queue.lock().unwrap().push_front(Some(x));
            // by unparking this thread, it will be able to trigger the thread to wake up
            // so that is will print all the items it needed
            t1.thread().unpark();
            // having sleep or not should not affect the thread in this scenerio
            thread::sleep(Duration::from_millis(500));
            if x == 10 {
                queue.lock().unwrap().push_front(None);
                // its important to call thread unpark or else the thread could be stuck in park if
                // the filan value is not handled for Some of None.
                t1.thread().unpark();
            }
        }
        // this wont work as it only drops the thread handle if we need to stop the thread
        // drop(t1);
        println!(" we have final for queue {:?}", queue.lock().unwrap());
    });
}




pub fn thread1c_arc_mutex() {
    #[derive(Debug)]
    struct TestMutex<'a> {
        a: std::sync::Mutex<u8>,
        b: std::sync::Mutex<u16>,
        c: std::sync::Mutex<&'a str>,
    }
    let x = Mutex::new(33);
    let y = Mutex::new(88);
    let z = Mutex::new("Hello there");
    // x,y,z are all moved the tmutx here
    let tmutx = TestMutex { a: x, b: y, c: z };
    println!("struct TestMutex before: {:#?}", tmutx);

    // this will give us a locked value of data
    let x_ = tmutx.a.lock();
    println!("struct TestMutex after: {:#?}", tmutx);
    drop(x_);

    println!("\n-------------------------Arc move------------------------------");
    let a = Arc::new(33);
    let b = Arc::clone(&a);
    let c = Arc::clone(&a);

    thread::spawn(move || {
        println!("b that is moved in this pointer arc is {}", b);
    })
    .join()
    .unwrap();

    thread::spawn(move || {
        println!("c is also moved in this arc is {}", c);
    })
    .join()
    .unwrap();

    // we can omit creating var like b,c by calling it inside the scope to make it cleaner
    let jn = thread::spawn({
        let a = a.clone();
        move || {
            println!("new a arch is {}", a);
        }
    });

    jn.join().unwrap();

    println!("\n-------------------------Arc Looping------------------------------");
    let x = std::sync::Arc::new(std::sync::Mutex::new(33u64));
    let y = std::sync::Arc::clone(&x);
    let z = std::sync::Mutex::new(vec![1, 2, 3, 4, 5]);
    let func = || {
        // y is passed here as reference, not moved
        // cause closures in itself dont need to have static
        // reference lifetimes. This is only a requireemnt
        // on thread spawn. If we however call thread spawn
        // inside of a thread scope object, it is able o use
        // this reference as it seen in func and func2
        // if let Ok(ref mut guard) = y.lock() { // this works too and we take **guard+=x;
        if let Ok(mut guard) = y.lock() {
            // for x in 1..=1_000_000_000 {
            // todomanish: see if we can use rayon here
            for x in 1..=1_000 {
                *guard += x;
            }
            println!("mutex mutated in spawned thread for y is {}", guard);
        } else {
            println!("didnt get the lock for y");
        }
    };

    let func2 = || {
        // since y was taken as reference before for func,
        // y can be taken as ref again as y was dropped before I imagine
        // if let Ok(ref mut guard) = y.lock() { // also works
        if let Ok(mut guard) = y.lock() {
            // for x in 1..=1_000_000_000 {
            for x in 1..=1_000 {
                *guard += x;
            }
            println!("mutex mutated in scawned thread for new y is {}", *guard);
        } else {
            println!("didnt get the lock for y new");
        }
    };

    // this will not work. as spawn needs static lifetime.
    // we will have to use a scope thread that has spawn inside it.
    // thread::spawn(func);

    let func3 = || {
        loop {
            // can be same a lock, except that this will not hand for try_lock if lock not recieved
            // if let Ok(ref mut vec) = z.try_lock() {  // works
            if let Ok(mut vec) = z.try_lock() {
                println!("lock received for z");
                if let Some(val) = vec.get_mut(4) {
                    *val += 10;
                }
                println!("vec is {:?}", vec);
                break;
            } else {
                println!("lock not received");
                thread::sleep(std::time::Duration::from_millis(200));
            }
        }
    };

    // even for vecs, this will work for func4
    // let func4 = || {
    //     loop {
    //         // can be same a lock, except that this will not hand when lock is not recieved
    //         if let Ok(ref mut vec) = z.try_lock() {
    //             println!("lock received for z");
    //             if let Some(val) = vec.get_mut(4) {
    //                 *val+=10;
    //             }
    //             println!("vec is {:?}", vec);
    //             break;
    //         } else {
    //             println!("lock not received");
    //             thread::sleep(std::time::Duration::from_millis(200));
    //         }
    //     }
    // };

    let m = std::sync::Mutex::new(0);
    let n = std::sync::Arc::new(m);
    thread::scope(|s| {
        s.spawn(func);
        s.spawn(func2);
        s.spawn(func3);
        for x in 1..=10 {
            // 10 threads are spawned here. So its all okay to testing
            let n = std::sync::Arc::clone(&n);
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
                                println!("Loop {}: guard is now {} at {}", x, *guard, y);
                            }
                        }
                        break;
                    } else {
                        println!("didnt get lock for {x}, trying after some milli seconds");
                        thread::sleep(std::time::Duration::from_millis(200));
                    }
                }
            });
        }
    }); // all thread join here
    // Here, we take Arc -> into_inner => mutex. and into_inner whick also works
    // assert_eq!(Arc::into_inner(n).unwrap().into_inner().unwrap(), 200);
    // lock should be dropped automatially after this scope
    assert_eq!(*n.lock().unwrap(), 300);
}
