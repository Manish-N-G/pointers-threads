//! This is a simple module that is used to understand how we can
//! use box to leak memory in order to pass static references for
//! a thread.
//!
//! These selected few functions will do this with box leak, 
//! and refcell interior mutability. However, what is important to 
//! know is that box leak will still give us the address of the leaked
//! memory while the refcell forget function wont be able to provide 
//! a reference. Its worth studying and understanding how this works.
use std::cell::RefCell;
use std::fmt::Debug;
use std::rc::Rc;
use std::thread;

// NOTE: done create MyNums<'a> as files will not hold 'a, 
// rather, we need to do for<'a> Self for reference of types
// holding 'a
/// The MyNums traits its just our traits that we have implemented
/// to represent all signed and unsigned integers from 1byte to 16bytes.
///
/// We have so make sure that certain conditions are present for this
/// trait type to unsure atleast as basic amount of operations and
/// trait bounds so that it can be accepted as numbers would.
/// We use this trait to create a generic type, that we will be using
/// for our lib.
/// There are also already other lib creates available like this that
/// allow us to use numbers like 
/// * num
/// * num-traits
/// 
/// However, is better it implement our own to understand and challenge
/// ourselves. This will quite simple and will not have some methods, 
/// if at all any, to help us like the other lib creates.
/// We have made this pub for the moment
pub trait MyNums
where Self: std::fmt::Debug + std::fmt::Display + Copy +  Default,
      Self: std::ops::Add<Output = Self>, 
      Self: std::ops::Sub<Output = Self>, 
      Self: std::ops::Mul<Output = Self>, 
      Self: std::ops::Div<Output = Self>, 
      Self: Send + Sync,
      Self: 'static,
      Self: TryInto<isize>,
      Self: TryInto<usize>,
      Self: Sized,
      for<'a> Self: std::iter::Sum<&'a Self>,
      for<'a> Self: std::iter::Product<&'a Self>
{
    fn as_usize(self) -> usize {
        // Safety: We can use unwrap here, knowing we will get 0
        // but looking at the implementation, we know that 0
        // is okay. We have to be careful about signed values
        self.try_into().unwrap_or(0usize)
    }
    fn as_isize(self) -> isize {
        // Safety: We can use unwrap here, knowing we will get 0
        // but looking at the implementation, we know that 0
        // is okay. We have to be careful about signed values
        self.try_into().unwrap_or(0isize)
    }
}

impl MyNums for u8{}
impl MyNums for u16{}
impl MyNums for u32{}
impl MyNums for u64{}
// impl MyNums for u128{}
impl MyNums for usize{}
impl MyNums for i8{}
impl MyNums for i16{}
impl MyNums for i32{}
impl MyNums for i64{}
// impl MyNums for i128{}
impl MyNums for isize{}


pub fn thread1b_box<I, T>(val: I, printable: bool) -> usize
where I: Iterator<Item = T>,
      T: MyNums,
{
    // let thbox = Box::new((1..=10).collect::<[i32]>()); // doesnt work
    let thbox = Vec::from_iter(val).into_boxed_slice(); //eg Box<[u16]>

    // doesnt work
    // let numbis mainly used for thread creationers = &mut thbox[..];
    // thread::spawn(move|| { // error on lifetimes
    //     for x in numbers {
    //         println!("val is {}",x);
    //     }
    // });

    // I write the type directly and not rely on the compiler. It give me an idea of how
    // they work too. Static type only accpeted as reference in standalone spawn.
    let numbers: &'static mut [T] = Box::leak(thbox);
    let th = thread::spawn(|| {
        for x in numbers.iter() {
            println!("val is {}", x)
        }
        let len = numbers.len();
        let sum = numbers.iter().sum::<T>();
        sum.as_usize() / len // this thread gives us the average
    });
    if printable {
        println!("numbers is {:?}", numbers);
    }
    let avg = th.join().unwrap();
    if printable {
        println!("avg is {}", avg);
    }
    avg
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
