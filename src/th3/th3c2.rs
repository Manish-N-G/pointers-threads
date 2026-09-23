use std::sync::Arc;
use std::sync::atomic::*;
use std::thread;

pub fn ordering_types() {
    let mut counter = 0u32;
    for _ in 0..1_000_000 {
        if run_ordering(
            Ordering::Relaxed,
            Ordering::Relaxed,
            Ordering::Relaxed,
            Ordering::Relaxed,
        ) == 0
        {
            counter += 1;
        }
    }

    println!("count is {}", counter);
}

struct SharedAtomics {
    data: AtomicU32,
    ready: AtomicBool,
}

fn run_ordering(order1a: Ordering, order1b: Ordering, order2a: Ordering, order2b: Ordering) -> u32 {
    let shared = Arc::new(SharedAtomics {
        data: AtomicU32::new(0),
        ready: AtomicBool::new(false),
    });

    let s = Arc::clone(&shared);
    let writer = thread::spawn(move || {
        s.data.store(42, order1a);
        s.ready.store(true, order1b);
    });

    let s = Arc::clone(&shared);
    let reader = thread::spawn(move || {
        while !s.ready.load(order2b) {
            // tells the processor that it can save power and optimize 
            // its looping to be a bit more efficient. Its not blindly 
            // running like crazy.
            std::hint::spin_loop();
        }

        s.data.load(order2a)
    });

    // I dont want to use write join
    // writer.join();
    reader.join().unwrap()
}


