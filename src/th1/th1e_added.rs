use std::sync::{Arc, RwLock};
use std::thread;
use std::time::Duration;

pub fn thread1e_arc_rwlock() {
    #[derive(Debug)]
    struct RwTest<'a> {
        a: RwLock<u8>,
        b: RwLock<u16>,
        c: RwLock<&'a str>,
    }
    // RwTest directly takes ownership and didnt need to move it in struct
    let st = RwTest {
        a: RwLock::new(11),
        b: RwLock::new(22),
        c: RwLock::new("Rw testing"),
    };
    println!("Testing Struct for RwLock {:#?}", st);

    let rwl = RwLock::new(33);
    {
        // can have many read locks. And this works across threads.
        // but I think the lock is made available immediately when we call unwrap.
        let r1 = rwl.read().unwrap();
        let r2 = rwl.read().unwrap();
        println!("rw R1 read is {:?}", r1);
        println!("rw R2 read 2 is {:?}", r2);

        // these following two causes problems
        // Since write will not panic if it doesnt get the lock, it will
        // wait/block till all readers release their lock
        // But since the lock is still held in this scope, this thread is made to
        // sleep indefinitely cause the thread is waiting to be released at the end of
        // the scope.
        // let r3 = rwl.write().unwrap();
        // println!("rw r3 is {:?}", r3);

        // However if we drop the reader locks before calling the write lock, we
        // should be able to get the reader
        drop(r1);
        drop(r2);
        // if we dont drop the r1 and r2 before hand, this will cause the thread to hang
        let mut r3 = rwl.write().unwrap();
        *r3 += 1;
        println!("rw R3 for manual drop is {:?}", r3);
    }
    let mut r3 = rwl.write().unwrap();
    *r3 += 1;
    println!("rw R3 after scope is {:?}", r3);

    println!("---------------------- RW lock struct to scope ---------------------------");

    // the \ char can make the string literal in a single line. It just says ignore the new line
    println!(
        "The order can change randomly for multiple threads. \
              Even thread sleep will not fix it. \
              Unless we join it manually with scopethreadhandle as I have done it for x"
    );
    let t = RwLock::new(44);
    thread::scope(|s| {
        // as we can have many reads, this works not just in a single thread but also
        // across threads. So this should be file.
        // So the purpose to having sleep in this case would be that the threads will start,
        // will be able to call read immediately.
        // However write will have to wait till all thre readers release the lock
        // Suppose, in this cause, x will start 1st as we have scopethreadhandle. join. cause
        // this is called before the 2 other get spawned.
        // When depending on the order, if read is received 1st, the write will have to wait
        // till that reader thread releases the reader lock. Assuming that will be 5 seconds for x
        // then 10 seconds for the reader thread read 2, we will have to wait that long will the
        // write get the change to get the thread. Once RWReadLock is droppen, it will notify that
        // its dropped. This is how the Write can get the thread of no other thread is accessing
        // that RWLock value
        let x = s.spawn(|| {
            let g = t.read().unwrap();
            println!("t rwlock read 1: {g}");
            thread::sleep(Duration::from_millis(2000));
        });
        // this will exe 1st and then the other 2 will run in random order.
        x.join().unwrap();
        s.spawn(|| {
            let mut g = t.write().unwrap();
            println!("t received the write lock");
            // This will we will have to wait 12 seconds will finally we can update the value
            thread::sleep(Duration::from_millis(5000));
            *g += 100;
            println!("t rwlock write 1 with +100 is: {g}");
        });
        s.spawn(|| {
            let g = t.read().unwrap();
            println!("t rwlock read 2: {g}");
            thread::sleep(Duration::from_millis(4000));
        });
    });

    println!("---------------------- RW lock in Arc ---------------------------");

    let lock = Arc::new(RwLock::new(55));
    let c_lock = Arc::clone(&lock);

    let n = lock.read().unwrap();
    assert_eq!(*n, 55);

    println!("th current id {:?}", thread::current());

    thread::spawn(move || {
        // since we called spawn, and not scope, we have to move the value inside
        let r = c_lock.read();
        // This allows us to see if we have the lock
        assert!(r.is_ok());
        println!(
            "th spawned current id {:?} and read val {:?}",
            thread::current(),
            r
        );
    })
    .join()
    .unwrap();

    let rw = Arc::new(RwLock::new(66));
    let crw = Arc::clone(&rw);
    // since we dont call unwrap on this, the compiler/clippy warns us that we
    // need to assign is something. If we call unwrap on this, this is not bug
    // us. I left this as a titbit for information
    let _ = thread::spawn(move || {
        let _lock = crw.write().unwrap();
        panic!("manually panicing this spawned");
    })
    .join();
    // even if thread panicked, we can still continus as its not the main thread
    // NOTE: calling panic on the main thread will stop the program unlike a thread
    // This below will cause this
    // panic!("haha panic main"); // nothing below will run
    println!("---------------------- After manual panic ---------------------------");
    assert!(rw.is_poisoned()); // will panic main if false
    println!("rw is poisonned and is rw is {:?}", rw);

    // Because rw is poisoned, we get the err version.
    // Here err is still poisoned version of RWWriteGuard, so we can still
    // use this to reassign/manuplate the value of the RwLock value
    let guard = rw.write().unwrap_or_else(|mut e| {
        // Err of Poisoned value of RWLock.
        **e.get_mut() = 1;
        // here is important to clear poison. The value will still be poisoned if we dont
        // revove this. Cause the struct will say poisoned = true,
        rw.clear_poison(); // poisoned = false
        e.into_inner() // This will give us new RWLock
    });
    assert!(!rw.is_poisoned());
    assert_eq!(*guard, 1);
    println!("rw after poison clear {}", guard);
}

pub fn thread1e_rwlock_is_finished() {
    println!("------------------------- is finished for RWlock--------------------");
    let x = std::sync::RwLock::new(1u64);
    let y = std::sync::Arc::new(x);
    let mut h = vec![];

    for a in 0..10 {
        let z = y.clone();
        h.push(std::thread::spawn(move || {
            // This also produces the same result. Except that its slower. How this works is that
            // when the thread read/write guard is not available, the thread is put to sleep
            // internally for the read/write. This is how read write runs. This sleep is woken by
            // roughly by how locks are taken for read/write. If the lock is not available, the
            // thread is put into a wait queue(Os parks the thread). And when another thread makes
            // the lock guard free and available, the next thread is selected from the list of
            // waiting threads. Eventually we wait till the list in the queue is all accounted for.
            // if a%3!=0 {
            //     if let Ok(val) = z.read() {
            //         println!("Val is now {} for loop {a}", val);
            //     }
            // } else {
            //     if let Ok(mut val) = z.write() {
            //         for _ in 0..1000000000 {
            //             *val += 1;
            //         }
            //     }
            // }
            if a % 3 != 0 {
                loop {
                    if let Ok(val) = z.try_read() {
                        println!("Val is now {} for loop {a}", val);
                        break;
                    }
                }
            } else {
                loop {
                    if let Ok(mut val) = z.try_write() {
                        for _ in 0..1000000000 {
                            *val += 1;
                        }
                        break;
                    }
                }
                // these 2 can be always dangerous. so dont do this
                // let o = z.write().unwrap();
                // let p = z.write().unwrap();
            }
        }));
    }

    h.iter().for_each(|handle| {
        loop {
            match handle.is_finished() {
                true => break,
                _ => std::thread::sleep(std::time::Duration::from_millis(100)),
            }
        }
    });
    // for x in h {
    //     println!("thread no {:?}", std::env::current_dir().unwrap());
    //     x.join().unwrap();
    // }
    println!(
        "Done for is_finished. We have final value {}",
        y.read().unwrap()
    );
}
