use std::thread;
pub mod th1a;
pub mod th1b;
pub mod th1c;
pub mod th1d;
pub mod th1e;
pub mod th1f;
pub mod th1g;

pub fn thread1st() {
    let th1 = thread::spawn(th1_func1);
    let th2 = thread::spawn(th1_func1);

    println!("Main thread after spanning this tread");
    println!("generally the threads start immediately");

    th1.join().unwrap();
    th2.join().unwrap();
    println!("Main thread id is {:?}", thread::current().id() );
}

fn th1_func1() {
    println!("Running spawned theread here");
    let id = thread::current().id();
    println!("This thread id is {:?}", id);
}
