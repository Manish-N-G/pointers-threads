//! This module is very similar to the one we did in [`crate::lib_ptr_a`].
//!
//! We mainly concentrate on the MySelfReferencePinned here for this 
//! module. We simple explain how we can use a `State Machine` Pattern
//! to prevent getting errors. This way, we know that out type, which
//! is a self reference, can expose certain methods only if we are in
//! at a perticular state.
//!
//! Make sure you have the change to brust up on how the following work 
//! - [`std::marker::Unpin`] and `!Unpin` trait bounds.
//! - [`std::pin`]
//! - [`std::pin::Pin`]
//! - [`std::pin::pin`]
//! - [`std::boxed::Box::pin`]
//! - [`std::marker::PhantomPinned`].
//!
//! This state machine will be using other internal Structs in order 
//! to make this more robust.
//!
//! Also to make our type a bit more interesting, We will make our 
//! struct a number type, so this way, it becomes a generaic over
//! `Nums`. Look at the [`crate::MyNums`] trait to see more

use crate::MyNums;

/// This SelfRefState type is taken inspiration from the 
/// [`crate::MySelfReferencePinned`] type. Like the `MySelfReferencePinned`
/// type, we will use `PhantomPinned`, to make sure that we annotate
/// this type with !Unpin trait properties.
///
/// We introduce a state machine pattern that starts from this type,
/// in order to make this public, while the other structs are kept 
/// private in generating it, but rather, allow us to call methods
/// that help guild the states.
///
/// Here, how this works with async a lot better, we also manually use the 
/// `Box::pin` type for make it simple, and not the pin::new and pin! operations.
///
/// Also, we will not restrict our type to a single type, but to a
/// whole number system that is defined by the `MyNums` trait.
/// # Todo:
/// `Coming Soon`: Async with MySelfReference types
pub struct MySelfRefState<T: MyNums> {
    val: T,
}


/// The pupose for this is maninly to expose the type to the user,
/// to ensure only certain methods that allows control of how a
/// seft ref type is used
/// # Todo:
impl<T: MyNums> MySelfRefState<T> {

    /// Create new MySelfRefState type
    /// ```
    /// # use pointers_threads::lib_ptr_a2::*; 
    ///
    /// let mut my_ref = MySelfRefState::new(3u8);
    ///
    /// // Here, we use the transmute function to indirectly test and
    /// // verify that we have 3. However, be careful when we use this
    /// // style of testing, while keeping in mind of how memory
    /// // allocation is done.
    /// assert_eq!( 3u8, unsafe { std::mem::transmute::<MySelfRefState<u8>, u8>(my_ref) } );
    /// ```
    pub fn new(val: T) -> Self {
        Self { val }
    }

    // NOTE: "cargo test --doc MySelfRefState" will work for all the tests
    // However. "cargo test --doc MySelfRefState::put_ptr" and other will not
    // work cause of the way cargo test for docs is run.
    // This will run thought:
    // cargo test --doc 'pointers::lib_ptr_a2::MySelfRefState' -- --nocapture
    // and cargo test --doc 'MySelfRefState' -- --nocapture
    // The fix: put them in quotes, with the <T> generic
    // cargo test --doc 'pointers::lib_ptr_a2::MySelfRefState<T>::put_ptr' -- --nocapture
    // cargo test --doc 'MySelfRefState<T>::put_ptr' -- --nocapture
    // 
    /// This function changes the state pattern to the 
    /// `Box::Pin(MySelfRefStatePin)` type. Here the address
    /// gets setup automatically on addition for this type.
    /// ```
    /// use pointers_threads::lib_ptr_a2::*;
    ///
    /// let my_ref = MySelfRefState::new(3u8);
    ///
    /// // will create a new pin state that is new struct of type
    /// // MySelfRefStatePin
    /// let my_pin = my_ref.put_ptr();
    ///
    /// let ( _, val, ptr) = unsafe { my_pin.get_addresses() };
    ///
    /// assert_eq!( val, ptr );
    ///
    /// ```
    pub fn put_ptr(self) -> std::pin::Pin<Box<MySelfRefStatePin<T>>> {
        let _my_ref = MySelfRefStatePin { 
            val: self.val,
            ptr: std::ptr::null(),
            _mkr: std::marker:: PhantomPinned
        };

        // This is put in a Box Pin type and the addresses
        // are updated to match each other. This way we ensure that
        // we dont have changes to making errors in the addresses
        let my_pin = Box::pin( _my_ref );
        // this was wrong
        // let pointer_val = &my_pin.val;
        // let mut _pointer_ptr = my_pin.ptr;
        // _pointer_ptr = pointer_val;
        // my_pin
        // this is also wrong as we have PhantomPinned making our type !Unpin
        // let pointer_val = &raw const my_pin.val;
        // my_pin.as_mut().get_mut().ptr = pointer_val;

        let pointer_val = &raw const my_pin.val;
        let _pointer_ptr = &raw const my_pin.ptr as *mut *const T;
        unsafe {*_pointer_ptr = pointer_val;}
        my_pin
    }
}


pub struct MySelfRefStatePin<T: MyNums> {
    pub val: T,
    pub ptr: *const T,
    // this converts to !Unpin type
    pub _mkr: std::marker::PhantomPinned,
}


/// I will have to do some unsafe impl here.
/// NOTE: This havs to be to type &self not &mut self. 
/// Or else PhantomPinned marker will complain and not
/// allow you to do this. This is done specifically for
/// safety. Our implementation is unsafe and this should 
/// not be the way this is done. But its good know that 
/// we could find a work aroudn it.. However, dont
/// implement is this way.
/// ```
/// use pointers_threads::lib_ptr_a2::*; 
///
/// let mut my_ref = MySelfRefState::new(3u8);
///
/// let my_ref_state = my_ref.put_ptr();
///
/// // assert_eq!(3u8, my_self_ref.get_val());
/// //todo:
/// ```
/// Also, for this type, since we have PhantomPinned type, we should 
/// avoid using ptr manipulation in order to access values.
/// **NOTE: These types are only meant to be used with Box::pin
/// but for the sake of understanding why it could cause problems,
/// we will also look at pin::new ( even if its not possible at the start)
/// at pin! macro.**
impl<T: MyNums> MySelfRefStatePin<T> {

    /*
    /// We get the value via ptr from the type.
    /// Dont forget to do `put_ptr_cast`
    /// ```should_panic
    /// # use pointers_threads::lib_ptr_a2::*;
    ///
    /// let mut my_self_ref = MySelfReferencePinned::new(3u8);
    ///
    /// assert_eq!( 3, my_self_ref.get_val() );
    /// ```
    /// Should be 
    /// ```
    /// # use pointers_threads::lib_ptr_a2::*;
    ///
    /// let mut my_self_ref = MySelfReferencePinned::new(3u8);
    ///
    /// unsafe {my_self_ref.put_ptr_cast(); }
    ///
    /// assert_eq!( 3, my_self_ref.get_val() );
    /// ```
    */
    pub fn get_val_by_ptr( &self ) -> T {
        // We get the value via pointers
        unsafe { *self.ptr }
    }

    /*
    /// update_val for the SelfReferencePinned type that is meant
    /// to be used before Pinning is done
    ///
    /// # Warning
    /// Despite being able to update_val, we still have to proceed with
    /// caution.
    /// ```
    /// use pointers_threads::lib_ptr_a2::*;
    ///
    /// let mut my_self_ref = MySelfRefPin::new(3u8);
    ///
    /// my_self_ref.update_val(8u8);
    /// ```
    /// Instead do this
    ///
    /// ```
    /// use pointers_threads::lib_ptr_a::*;
    ///
    /// let mut my_self_ref = MySelfReference::new(3u8);
    /// // Dont forget to use put_ptr 1st
    /// my_self_ref.put_ptr();
    /// let (val, ptr) = my_self_ref.get_addresses();
    /// assert_eq!( val, ptr );
    ///
    /// my_self_ref.update_val(8u8);
    /// let (val, ptr) = my_self_ref.get_addresses();
    /// assert_eq!( val, ptr );
    ///
    /// my_self_ref.update_val(18u8);
    /// let (val, ptr) = my_self_ref.get_addresses();
    /// assert_eq!( val, ptr );
    ///
    /// ```
    */
    pub fn update_val_by_ptr(&self, val: T) {
        let ptr_val = &raw const self.val as *mut T;
        unsafe { *ptr_val = val; }
    }
    

    /// To get the addresses for the value and the ptr raw
    /// address ptr. This is unsafe, and we have to proceed
    /// with caution
    /// ```
    /// use pointers_threads::lib_ptr_a2::*;
    ///
    /// let my_self_ref = MySelfRefState::new(3u8);
    /// let my_self_ref_to_pin = my_self_ref.put_ptr();
    /// let (_, val, ptr) = unsafe { my_self_ref_to_pin.get_addresses() };
    /// assert_eq!( val, ptr );
    /// ```
    // NOTE:
    // The following will not compile as we see that its not possible
    // to fine the get_addresses function. We dont use compile_fail, but
    // we use the ignore one here
    /// ```ignore
    /// // compile_fail when we only have compile issues.
    /// // ```compile_fail
    /// // this will compile, so we dont use no_run
    /// // ```no_run
    /// use pointers_threads::lib_ptr_a2::*;
    ///
    /// let mut my_self_ref = MySelfRefState::new(3u8);
    /// let (_, val, ptr) = my_self_ref.get_addresses();
    /// assert_eq!( val, ptr );
    /// ```
    /// # Safety
    /// something to do
    pub unsafe fn get_addresses(&self) -> ( &Self, &T, &T) {
        unsafe {( self, &self.val,  &*self.ptr ) }
    }

    /*
    /// Prints the address for the type and its fields and inner fields.
    /// ```should_panic
    /// // Some command that could help
    /// // cargo test --doc -- --list
    /// // cargo test --doc MySelfReferencePinned::print_addr
    /// // cargo test --doc "MySelfReferencePinned::print_addr"
    /// // NOTE: --nocapture and --show-output causes issue for doc tests
    /// // cargo test --doc MySelfReferencePinned::print_addr -- --nocapture
    ///
    /// use pointers_threads::lib_ptr_a::*;
    ///
    /// let mut my_self_ref_to_pin = MySelfReferencePinned::new(3u8);
    /// my_self_ref_to_pin.put_ptr();
    /// my_self_ref_to_pin.print_addr("Extra condition");
    ///
    /// // to see print statements, we manually panic
    /// panic!("Manual Panic here for print_addr -------------");
    /// ```
    */
    pub fn print_addr(&self, condition: &str) {
        if !condition.is_empty() {
            println!("{condition}");
        }
        let ( var, val, ptr) = unsafe { self.get_addresses() };
        println!("add of variable is {:p}", var);
        println!("add of val is {:p}", val);
        println!("add of ptr is {:p}\n", ptr);
    }
}


// NOTE: Despite being available for Send, we have to proceed
// with caution.
// unsafe impl Send for MySelfReferencePinned {}
// NOTE: Despite being available for Sync, we have to proceed
// with caution.
// unsafe impl Sync for MySelfReferencePinned {}


