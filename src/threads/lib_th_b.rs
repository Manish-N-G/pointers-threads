//! This is a simple module that is used to understand how we can
//! use box to leak memory in order to pass static references for
//! a thread.
//!
//! These selected few functions will do this with box leak, 
//! and refcell interior mutability. However, what is important to 
//! know is that box leak will still give us the address of the leaked
//! memory while the refcell forget function wont be able to provide 
//! a reference. Its worth studying and understanding how this works.
//!
//! The general implementation to something like this is the following
//! ```
//! use std::thread;
//! // eg: just a simple vector
//! let v = vec![1u8,2,3];
//! let box_v_slice = v.into_boxed_slice(); //eg Box<[u8]>
//!
//! // its good to sometimes explicitely say its 'static
//! let leaked_pointer: &'static mut [u8] = Box::leak(box_v_slice);
//! let t1 = thread::spawn(|| {
//!     leaked_pointer.iter().sum::<u8>() as usize
//! });
//!
//! assert_eq!(t1.join().unwrap(), 6usize);
//! ```
//!
//! If we want to leak memory through reference counting, its best not
//! to do it as we will not get anything worth doing at all from it.
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
      for<'a> Self: std::iter::Product<&'a Self>,
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

/// be careful for lower values, as overflow is possible
impl MyNums for u8{}
/// be careful for lower values, as overflow is possible
impl MyNums for u16{}
/// even for larger value, over flow is still possible
impl MyNums for u32{}
/// even for larger value, over flow is still possible
impl MyNums for u64{}
// impl MyNums for u128{}
/// even for larger value, over flow is still possible
impl MyNums for usize{}
/// be careful for lower values, as overflow is possible
impl MyNums for i8{}
/// be careful for lower values, as overflow is possible
impl MyNums for i16{}
/// even for larger value, over flow is still possible
impl MyNums for i32{}
/// even for larger value, over flow is still possible
impl MyNums for i64{}
// impl MyNums for i128{}
/// even for larger value, over flow is still possible
impl MyNums for isize{}


/// This takes a iterator and Boxes is on the heap. I later leaks
/// the memory, so that we can pass it to the thread
///
/// We should not be doing this, as this will unnecessary cause
/// **memory leads** just to compute a sum of iterator for *Generic 
/// ***'T'****
/// However, this still illustrates how this works, and we can be aware
/// of it as a for or passing values statically to a thread.
///
/// The way we can implement this is simple
/// ```
/// use std::thread;
/// use pointers_threads::lib_th_b::*;
///
/// // produces a boxed slice via 
/// // let v = into_boxed_slice for our (1..=10)
/// // then it leaks its memory and get pointer via
/// // let addr: & static mut i32 = Box::leak(v);
/// // This is then passed in the thread and we get the
/// // value we wanted via sum of iter to produce our avg.
/// assert_eq!(thread1b_box_leak_avg(1..=10), 5usize);
/// // 5 is the avg for 1 to 10 for rounder values
/// ```
pub fn thread1b_box_leak_avg<I, T>(val: I) -> usize
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
        // even if we pub this in the lib, which is not required,
        // we can debug the value here in doc tests
        // for x in numbers.iter() {
        //     println!("val is {}", x)
        // }
        
        let len = numbers.len();
        // remember that sum here is wrapping type, but will
        // panic in debug mode if beyond the T limit.
        let sum = numbers.iter().sum::<T>();
        sum.as_usize() / len // this thread gives us the average
    });
    th.join().unwrap()
}

/// This functions show that we can also leak memory if we
/// wanted through reference cycles.
///
/// We should not be doing this as well, there is no need for it
/// and we will get nothing useful from it. This functions only
/// purpose it to leak memory without getting any pointer to the
/// data also.
///
/// How this can help us is to understand what we will need to avoid
/// in order to prevent memory leaks in production. 
/// ```
/// // for creating a memory leak via reference cycles, we need 
/// // implement a drop implementation on a struct and this will
/// // create the memory leak we should be trying to avoid.
///
/// use std::rc::Rc;
/// use std::cell::RefCell;
///
/// struct Foo<T>(T, RefCell<Option<Rc<Foo<T>>>>);
///
/// struct DontDropMe;
/// // then; when we drop has the potential to
/// // memory as the values are not cleared by for the 
/// // type that is struct in a reference cycle
/// impl Drop for DontDropMe {
///     fn drop(&mut self) {
///         unreachable!();
///     }
/// }
///
/// let val = DontDropMe;
/// let x = Rc::new(Foo(val, RefCell::new(None)));
///
/// // we create a reference cycle here
/// *x.1.borrow_mut() = Some(Rc::clone(&x));
/// ```
///
/// For our purposes, we dont need to do much testing. I can 
/// just expose this function, and we can look and try to
/// see hows its implemented
pub fn thread1b_forget_leak() {
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
