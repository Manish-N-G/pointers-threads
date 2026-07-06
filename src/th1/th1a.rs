use std::thread;

pub fn thread1a_builder() {
    let v = vec!['a', 'b', 'c'];
    // This thread Builder constructs a thread, but allows to
    // define some traits about it like giving it a name.
    let t2 = thread::Builder::new().name("Puchi".into()).spawn(move || {
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
        for x in stat.chars() {
            // possible as its static lifetime and chars borrows self
            print!("{} ", x);
        }
        println!();
    })
    .join()
    .unwrap();
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
