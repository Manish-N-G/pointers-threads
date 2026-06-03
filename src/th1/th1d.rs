use std::thread;
use std::sync::{Arc,Mutex};
use std::collections::vec_deque::VecDeque;
use std::time::Duration;

pub fn thread1d_mutex_lock_attempt() {

    let x = Mutex::new(11u8);
    // this is just to illustrate that mutex lock, when not received for Lock(), doesnt mean that
    // the tread will panic, instead the thread is put to sleep while waiting for the lock to
    // be released. Here, the lock is never released before the other lock() is called, and hence
    // this will be hung forever and called be passed on.
    thread::scope(|s| { 
        let th1 = s.spawn(|| {
            println!("Before calling double lock");
            {
                let y = x.lock().unwrap();
                println!("first lock received");
                drop(y); // if this drop was not here, this program will hang forever
                #[allow(unused)]
                let z = x.lock().unwrap();
                // drop(z); // This drop will happen automatically
            }
            println!("I doesn't panic, it just get stuck falls asleep if we dont/blocked if two locks are
                received at the same time");
        });

        for _ in 0..100 {
            thread::sleep(std::time::Duration::from_millis(100));
            println!("trying to unpark/wakeup");
            th1.thread().unpark();
            if let Ok(mut v) = x.try_lock() {
                println!("got the lock");
                *v +=1;
            } else {
                println!("trying to unpark/wakeup");
            }
        }
        println!("finished loop, if dont see any print after this for his function, thread failed, ");
    });
    println!("passed"); // this is not possible

    {
        println!("simple test for references");
        let mx = std::sync::Mutex::new(22u8);
        let mut v:Vec<u8> = vec![];
        let mxv = std::sync::Mutex::new(vec![1]);

        std::thread::scope(|s| {
            s.spawn(|| {
                let mut guard = mx.lock().unwrap();
                // but this is dangerous... 2 locks at the same time without dropping 1st one.
                // let mut guard = mx.lock().unwrap();
                *guard += 1;
                v.push(*guard); // this works but can un unsafe as ordering will cause issues
                                // if not accounted for. eg, we may push 23 or 24 fepending on
                                // which thread runs 1st
                let mut guard2 = mxv.lock().unwrap();
                guard2.push(*guard);
            });
            s.spawn(|| {
                let mut guard = mx.lock().unwrap();
                *guard += 1;
                // via captured variables it doesnt work, but through the mutex pointer
                // it does.
                // v.push(*guard); // this doesnt work
                let mut guard2 = mxv.lock().unwrap();
                guard2.push(*guard);
            });
        });

        println!("val is now {:?}", mx);
        println!("val is now v {:?}", mxv);

        let m = Arc::new(mx);
        let m1 = m.clone();
        let th1 = thread::spawn( move|| {
            let mut guard = m1.lock().unwrap();
            // but this is dangerous... 2 locks at the same time without dropping 1st one.
            // let mut guard = mx.lock().unwrap();
            *guard += 1;
        });
        let m1 = m.clone();
        let th2 = thread::spawn(move|| {
            let mut guard = m1.lock().unwrap();
            *guard += 1;
        });
        th1.join().unwrap();
        th2.join().unwrap();
        println!("val is now {:?}", m);
    }

}


pub fn thread1d_park_mutex() {
    // this is just to illustrate that mutex lock, when not received for Lock(), doesnt mean that
    // the tread will panic, instead the thread is put to sleep while waiting for the lock to
    // be released. Here, the lock is never released before the other lock() is called, and hence
    // this will be hung forever and called be passed on.
    // {
    //     let x = std::sync::Mutex::new(3);
    //     println!("Before calling double lock");
    //     {
    //         let y = x.lock().unwrap();
    //         println!("first lock received");
    //         let z = x.lock().unwrap();
    //     }
    //     println!("I doesn't panic, it just get stuck falls asleep if we dont/blocked if two locks are
    //         received at the same time");
    // }
    
    println!("---------------------------thread park for mutex-----------------------");
    let queue = Mutex::new( VecDeque::<Option<u8>>::new() );
    thread::scope(|s| { 
        // loop is called we have to get all the values from the vector
        let t1 = s.spawn(|| loop {
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
                Some(Some(val)) => { dbg!(val); },
                Some(None) => break,
                None => thread::park(),
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

pub fn thread1d_arc_mutex() {

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
    let tmutx = TestMutex { a: x, b: y, c: z};
    println!("struct TestMutex before: {:#?}", tmutx);

    // this will give us a locked value of data
    let x_ = tmutx.a.lock(); 
    println!("struct TestMutex after: {:#?}", tmutx);
    drop(x_);

    println!("\n-------------------------Arc move------------------------------");
    let a = Arc::new(33);
    let b = Arc::clone(&a);
    let c = Arc::clone(&a);

    thread::spawn(move|| {
        println!("b that is moved in this pointer arc is {}", b);
    }).join().unwrap();

    thread::spawn(move || {
        println!("c is also moved in this arc is {}", c);
    }).join().unwrap();

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
    let z = std::sync::Mutex::new(vec![1,2,3,4,5]);
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
            for x in 1..=1_000 {
                *guard+=x;        
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
                *guard+=x;        
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
                    *val+=10;
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
        for x in 1..=10 { // 10 threads are spawned here. So its all okay to testing 
            let n = std::sync::Arc::clone(&n);
            // since x would have been taken as refernece for print statement, we have to use
            // move cause we cant be sure that it will life long enougth
            // The compiler cant verify even it this is possible.
            // And because we will have to end up using move, we have to pass our data in Arc clone
            s.spawn( move || {
                loop {
                    // try lock will not block it tests if it gets the lock
                    // but we will have to keep attempting to get the lock which is why we put
                    // it in a loop
                    if let Ok(mut guard) = n.try_lock() {
                        for y in 1..=20 {
                            *guard+=1;
                            // to avoid too many print statements
                            if y%5==0 { println!("Loop {}: guard is now {} at {}",x, *guard, y); }
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

