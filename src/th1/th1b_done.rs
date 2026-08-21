use std::cell::RefCell;
use std::fmt::Debug;
use std::rc::Rc;
use std::thread;

pub fn thread1b_box() {
    // let thbox = Box::new((1..=10).collect::<[i32]>()); // doesnt work
    let thbox = Vec::from_iter(1u16..=10).into_boxed_slice(); // Box<[u16]>

    // doesnt work
    // let numbers = &mut thbox[..];
    // thread::spawn(move|| { // error on lifetimes
    //     for x in numbers {
    //         println!("val is {}",x);
    //     }
    // });

    // I write the type directly and not rely on the compiler. It give me an idea of how
    // they work too. Static type only accpeted as reference in standalone spawn.
    let numbers: &'static mut [u16] = Box::leak(thbox);
    let th = thread::spawn(|| {
        for x in numbers.iter() {
            println!("val is {}", x)
        }
        let len = numbers.len();
        let sum = numbers.iter().sum::<u16>() as usize;
        sum / len // this thread gives us the average
    });
    let avg = th.join().unwrap();
    println!("avg is {}", avg);
}

pub fn thread1b_forget() {
    fn forget<T: Debug>(val: T) {
        #[derive(Debug)]
        struct Foo<T>(T, RefCell<Option<Rc<Foo<T>>>>);
        let x = Rc::new(Foo(val, RefCell::new(None)));
        *x.1.borrow_mut() = Some(Rc::clone(&x));
        println!("be careful for forget. x.0 works {:?}", x.0);
        // println!("be careful for forget. x.1 doesnt works. will overflow {:?}", x.1);
        // this will create a cyclic reference and value is lost. does the same as leak for box
    }

    #[derive(Debug)]
    struct DontDropMe;
    impl Drop for DontDropMe {
        fn drop(&mut self) {
            println!("this should never be called");
            unreachable!();
        }
    }

    forget(DontDropMe);
}
