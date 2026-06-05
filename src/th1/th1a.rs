use std::thread;

/// Creates a spawned thread and Joins and returns the result
///
/// This is a simple test thread that takes a vector, and adds 
/// 6 to it at the end of the vec
/// ```
/// use std::thread;
/// let v = vec![1,2,3,4,5];
/// let a1 = thread1a(v);
///
/// // we cant do print statement event with --show-output/ --nocapture
/// assert_eq!(&a1, &[1,2,3,4,5,6])
/// ```
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

pub fn thread1a_builder() {
    let v = vec!['a', 'b', 'c'];
    // This thread Builder constructs a thread, but allows to
    // define some traits about it like giving it a name.
    let t2 = thread::Builder::new()
        .name("Puchi".into())
        .spawn(move||{
            println!("checking with name");
            println!("v is {:?}", v);
        });

    // This can give an result which is why we unwrap and then join
    _ = t2.unwrap().join();
    println!("thread builder is used here ");
} 

pub fn thread1a_stat() {
    let stat = "this is stat thread"; // static lifetime
    thread::spawn(|| {
        // Here stat is not moved but referenced in this closure
        for x in stat.chars() { // possible as its static lifetime and chars borrows self
            print!("{} ", x);
        };
        println!();
    }).join().unwrap();
    // this will allow me to use stat after calling it here
    println!("stat is {stat}");
}

pub fn thread1a_scope() {
    let v = (1..1000).collect::<Vec<u32>>();
    #[allow(warnings)]
    let mut v2 = ('a'..'z').collect::<Vec<char>>();
    // scope doesnt need to have reference of 'static lifetime. 
    // But what is important to know is that we cant have more
    // than 1 mutable referece instance in the scope, but serveral shared references.
    // lifetimes can be smaller than static as scope finished the tread 
    // by join at the end
    thread::scope(|s| {
        s.spawn(|| {
            for x in &v {
                print!("{}.", x);
            }
            println!();
        }); // these threads are run concurrently
        s.spawn(|| {
            for x in &v {
                print!("{}-", x);
            }
            println!();
        });
        // s.spawn(|| {
        //     v.push(3);
        // }) // this is not possible
        s.spawn(|| {
            v2.push('z'); // automatic borrow ( as mut ) occures here.
        });
        // s.spawn(|| {
        //     v2.push('z'); // automatic borrow ( as mut ) occures here.
        // }); // cannot be possible as we already called 1st mut ref of v2
    }); // all the threads are joined here.
}

