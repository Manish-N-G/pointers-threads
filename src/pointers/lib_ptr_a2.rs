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
/// this type with !Unpin trait properties when we change the state
/// to [`crate::MySelfRefStatePin`].
///
/// We introduce a state machine pattern that starts from this type,
/// in order to make this public, while the other structs are kept 
/// private in generating it, but rather, allow us to call methods
/// that help guild the states.
///
/// Here, how this works with async a lot better, we also manually use the 
/// `Box::pin` type for make it simple, and not the pin::new and pin! operations,
/// when we transform out type to `MySelfRefStatePin`. However, in order to
/// prevent Send and Sync for this `MySelfRefState` type, we introduce a
/// phantomData type.
///
/// Also, we will not restrict our type to a single type, but to a
/// whole number system that is defined by the `MyNums` trait.
pub struct MySelfRefState<T: MyNums> {
    val: T,
    // I prefer to make this type !Send and !Sync till we
    // change states.
    _mkr: std::marker::PhantomData<std::cell::Cell<u8>>,
}


/// The pupose for this is maninly to expose the type to the user,
/// to ensure only certain methods that allows control of how a
/// seft ref type is used
/// The two methods used are as follows
/// ```
/// # use pointers_threads::lib_ptr_a2::*; 
/// let mut my_ref = MySelfRefState::new(3u8);
///
/// // This state machine patter will not let this compile anymore.
/// // we will have to use the put_ptr first.
/// //let add = unsafe { my_ref.get_addresses() };
///
/// let mut my_ref_pin = my_ref.put_ptr();
/// 
/// // Now, we are able to make sure that we have the State used
/// // update values for Self References
/// 
/// assert_eq!( 3u8, unsafe { *(my_ref_pin.get_addresses().1) } );
///
/// ```
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
        Self { val, _mkr: std::marker::PhantomData }
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
    /// gets setup automatically on addition to making it Pin.
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
        // notice this doesnt work
        // let pointer_val = &my_pin.val;
        // let mut _pointer_ptr = my_pin.ptr;
        // _pointer_ptr = pointer_val;

        // this is also wrong as we have PhantomPinned making our
        // type !Unpin. Push Box Pin will have to be mut
        // let pointer_val = &raw const my_pin.val;
        // my_pin.as_mut().get_mut().ptr = pointer_val;

        let pointer_val = &raw const my_pin.val;
        let _pointer_ptr = &raw const my_pin.ptr as *mut *const T;
        unsafe {*_pointer_ptr = pointer_val;}
        my_pin
    }
}


/// This is a continuations from the [`crate::MySelfRefState`] struct
/// where, we get the MySelfRefStatePin types. We cannot manually 
/// create this type without gonig through the `MySelfRefState` struct
/// and this way, we are able to maintain the state and expose
/// only those methods that would be able to update the value safely
/// as well as be able to work with futures and threads without
/// worrying how the self reference type could incorrectly point to
/// the wrong address when we pass ownership.
///
/// We make use of the PhantomPinned type, that makes the type a 
/// `!Unpin' type. This will prevent us from moving this type to
/// a differnt memory address, via the help of Pinning
/// # Todo:
/// `Coming Soon`: Async with MySelfRefStatePin types
pub struct MySelfRefStatePin<T: MyNums> {
    val: T,
    ptr: *const T,
    // this converts to !Unpin type
    _mkr: std::marker::PhantomPinned,
}


// I will have to do some unsafe impl here.
/// We use the methods here the way we want to in order to
/// understand control how to update the value inside the 
/// pinned type.
/// NOTE: This havs to be to type &self not &mut self. 
/// Or else PhantomPinned marker will complain and not
/// allow you to do this. This is done specifically for
/// safety. Our implementation is unsafe and this should 
/// not be the way this is done. But its good know that 
/// we could find a work aroudn it.. However, dont
/// implement is this way.
/// ```
/// # use pointers_threads::lib_ptr_a2::*; 
///
/// let my_ref = MySelfRefState::new(3u8);
///
/// let mut my_ref_pin = my_ref.put_ptr();
///
/// assert_eq!( 3u8, unsafe { *(my_ref_pin.get_addresses().1) } );
/// assert_eq!( 3u8, my_ref_pin.get_val_by_ptr() );
///
/// my_ref_pin.update_val_by_ptr(10u8);
/// assert_eq!( 10u8, unsafe { *(my_ref_pin.get_addresses().1) } );
/// assert_eq!( 10u8, my_ref_pin.get_val_by_ptr() );
/// ```
/// ** These types are only meant to be used with Box::pin
/// which is why, we manually do it through the 
/// [`crate::MySelfRefState::put_ptr`] method. And the fields
/// make private on purpose.
impl<T: MyNums> MySelfRefStatePin<T> {

    /// We get the value via ptr from the type.
    /// This will not compile, as we didnt use `put_ptr`
    /// ```ignore
    /// # use pointers_threads::lib_ptr_a2::*;
    ///
    /// let mut my_self_ref = MySelfRefState::new(3u8);
    ///
    /// assert_eq!( 3, my_self_ref.get_val_by_ptr() );
    /// ```
    /// Here we added the put_ptr for make out type
    /// the `MySelfRefStatePin` type.
    /// ```
    /// # use pointers_threads::lib_ptr_a2::*;
    ///
    /// let my_self_ref = MySelfRefState::new(3u8);
    ///
    /// let mut my_pin = my_self_ref.put_ptr();
    ///
    /// assert_eq!( 3, my_pin.get_val_by_ptr() );
    /// ```
    pub fn get_val_by_ptr( &self ) -> T {
        // We get the value via pointers
        unsafe { *self.ptr }
    }

    /// Updates the value for the SelfRefStatePin type that
    /// can now only be done as this type is always `Pinned`.
    /// ```ignore
    /// # use pointers_threads::lib_ptr_a2::*;
    ///
    /// let mut my_self_ref = MySelfRefState::new(3u8);
    ///
    /// my_self_ref.update_val_by_ptr(8u8);
    /// ```
    /// But, now, via its state machine, we can do is this way
    ///
    /// ```
    /// use pointers_threads::lib_ptr_a2::*;
    ///
    /// let my_self_ref = MySelfRefState::new(3u8);
    /// // Dont forget to use put_ptr 1st
    /// let mut my_self_ref = my_self_ref.put_ptr();
    ///
    /// let (_, val, ptr) = unsafe { my_self_ref.get_addresses() };
    /// assert_eq!( val, ptr );
    ///
    /// my_self_ref.update_val_by_ptr(8u8);
    /// let (_, val, ptr) = unsafe { my_self_ref.get_addresses() }; 
    /// assert_eq!( val, ptr );
    ///
    /// my_self_ref.update_val_by_ptr(18u8);
    /// let (_, val, ptr) = unsafe { my_self_ref.get_addresses() }; 
    /// assert_eq!( val, ptr );
    ///
    /// ```
    pub fn update_val_by_ptr(self: &mut std::pin::Pin<Box<Self>>, val: T) {
    // pub fn update_val_by_ptr(&mut self, val: T) {
        // worked when we had &self, cause Pin<Box<Self<T>>>
        // doesnt implement derefmut
        
        let ptr_val = &raw const self.val as *mut T;
        unsafe { *ptr_val = val; }
    }
    

    /// To get the addresses for the value and the ptr raw
    /// address ptr. This is unsafe, as we should easily expose
    /// the pointers to values so easily after this type is
    /// already `Pinned`, and is `!Unpin`.
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
    /// We have to make sure that we Pin the value 1st
    /// ```
    /// # use pointers_threads::lib_ptr_a2::*;
    ///
    /// let my_self_ref = MySelfRefState::new(3u8);
    /// let my_self_ref_to_pin = my_self_ref.put_ptr();
    /// let (_, val, ptr) = unsafe { my_self_ref_to_pin.get_addresses() };
    /// assert_eq!( val, ptr );
    /// ```
    /// # Safety
    /// Here the Safety arguement is that, we will be able to 
    /// use the addresses for general testing, but the danger 
    /// lies in exposing the addresses easily. This will allow
    /// others to exploit this, which is not what we want for our
    /// safe State machine. Here, we use unsafe as well because
    /// we dereference raw pointers. Otherise, we are able to 
    /// get the addresses as expected.
    pub unsafe fn get_addresses(&self) -> ( &Self, &T, &T) {
        unsafe {( self, &self.val,  &*self.ptr ) }
    }

    /// Prints the address for the type and its fields and inner fields.
    /// ```should_panic
    /// // Some command that could help
    /// // cargo test --doc -- --list
    /// // cargo test --doc MySelfReferencePinned::print_addr
    /// // cargo test --doc "MySelfReferencePinned::print_addr"
    /// // NOTE: --nocapture and --show-output causes issue for doc tests
    /// // cargo test --doc MySelfReferencePinned::print_addr -- --nocapture
    /// # use pointers_threads::lib_ptr_a2::*;
    ///
    /// let mut my_self_ref_to_pin = MySelfRefState::new(3u8);
    /// let my_pin = my_self_ref_to_pin.put_ptr();
    /// my_pin.print_addr("Extra condition");
    ///
    /// // to see print statements, we manually panic
    /// panic!("Manual Panic here for print_addr -------------");
    /// ```
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


/// Our state machine of MySelfRefState does not yet have the 
/// Pin trait for it. And this type would expose our type as
/// Unpin, making it not the best for Sending and Syncing over
/// threads, even if it can be done safely. I Prefer to know have 
/// This as Send and Sync till we get the `MySelfRefStatePin` type.
/// unsafe impl<T: MyNums> !Send for `MySelfRefState<T>` {} here is 
/// nightly, but I used PhantomDate to set our type to !Send
/// and !Sync.
///
/// However, out state machine is now more secure to be passed between threads
/// or tasks, when it become `MySelfRefStatePin`, if we were to pass
/// this type, we know that state will not expose us to get the
/// value unsafely, which is our main goal.
unsafe impl<T: MyNums> Sync for MySelfRefStatePin<T> {}

//todomanish: Have to set up examples function for this as well.
