//! Testing ptr_a
//!
//!
//!

/// testing this
/// rust,editable ??
/// ```
/// //use pointers_threads::lib_ptr_a::*;
/// use pointers_threads::unsafe_raw_vector_element_mutability;
/// // use my_crate::assert_panic_message;
/// let a = vec![1usize, 2, 3];
///
/// // we have to be careful here when we pass the vec.
/// // this function is inherently made so that this will
/// // panic if the vec is less then 3 elements longs.
///
/// // However this function doesnt focus too much after
/// // on the value as it just simple manuplates the value
/// // by creating a new fixed one. We do this simple to show
/// // how const mut raw pointers can be used to perform 
/// // additions and casting.
///
/// // Its good to get a good idea how this works be looking
/// // directly at the code for this function
/// // Safety: This function is not save, as we cannot have more than 
/// // 2 values. We need to ensure safety, but I leave it as it is.
/// assert_eq!( unsafe {unsafe_raw_vector_element_mutability(a) }, ( 8208, 8209 ));
/// ```
/// # Safety
///
/// Just a note, this doesn't provide any safety and this function will panic if
/// the length of the vec is less than 3. Its a good way to demonstrate that that
/// unsafe functions have to be handled with care
pub unsafe fn unsafe_raw_vector_element_mutability(vec: Vec<usize>) -> (u16, u16) {
    // On purpose I used an unsafe get_unchecked element for
    // vector to illustrate how it could be used
    // Safety: There is not safe and this should not
    // be used. It will panic if vec is less than len 3. 
    // We do this just to show how we can get/manipulate
    // pointers
    let mut element: u32 = unsafe { vec.get_unchecked(2) }.to_owned() as u32;
    println!("a is {}", element);

    element = 2151686160;         //10000000 01000000 00100000 00010000
    #[allow(unused)]
    // we convert this to &u16 from u32
    let const_u16 = &element as *const u32 as *const u16; // 00100000 00010000
    let mut_u16 = &mut element as *mut u32 as *mut u16;
    //or
    let const_u16 = std::ptr::addr_of!(element) as *const u16; // 00100000 00010000 // 8208

    let (a, b) = unsafe { danger_pointer_val_inc(const_u16, mut_u16) };

    // dereferencing raw pointers are unsafe
    (a, b)
}

/// Takes a const ref and mut ref of some values. And increment the
/// deref of the 2nd arguement that is passed.
///
/// This was intended to pass the addr of the same value to show
/// how unsafe raw pointers to the same type can be done in a function
/// signature, as well of passing values to it.
///
/// ```
/// use pointers_threads::lib_ptr_a::*; 
///
/// // This works also for our method
/// let mut b:u16 = 33;
/// let c = &b as *const u16;
/// let d = &mut b as *mut u16;
/// // Caution: passing the following will panic cause b will
/// // become b = 34. then be comparing b and b +1 will produce an
/// // error and this needs to be considered
/// // assert_eq!( unsafe { danger_pointer_val_inc(c,d) }, (b, b+1));
/// // Safety: This function safe for types that dont exceed the
/// // limits of u16. In increaments via raw pointers
/// assert_eq!( unsafe { danger_pointer_val_inc(c,d) }, (b-1, b));
/// assert_eq!( unsafe { danger_pointer_val_inc(c,d) }, (34, 35));
///
/// // I could also do
/// let b:u16 = 33;
/// let c = &b as *const u16;
/// let d = c as *mut u16;
/// assert_eq!( unsafe { danger_pointer_val_inc(c,d) }, (33, 34));
///
/// // or even this
/// let b:u32 = 33;
/// let c = &b as *const u32 as *const u16;
/// let d = c as *mut u16;
/// assert_eq!( unsafe { danger_pointer_val_inc(c,d) }, (33, 34));
///
/// ```
/// # Safety
///
/// This function is marked as unsafe because we are trying to receive 
/// raw pointers, as arguements. This is important to know and be will
/// not be able to pass if without the unsafe tag. However, knowing this,
/// this doesnt mean that this function is unsafe. what it does it takes 
/// a const ref and mut ref of the same value and gives us the deref along
/// with increment of the deref.
pub unsafe fn danger_pointer_val_inc(a: *const u16, b: *mut u16) -> ( u16, u16 ){
    // as we are working with raw pointers, we can use unsafe
    unsafe {
        let before = *a; // val copied. Not a reference pointer
        *b += 1;
        let after = *a;
        if before != after {
            println!(
                "they are not equal as before {} is not as after {}",
                before, after
            );
        }
        (before, after)
    }
}

/// This struct shows a SelfReference type that has a value and a pointers.
/// The methods on this struct call on creating a Self Reference where the 
/// Option of the raw pointers points the value in this struct.
///
/// We use this to illustrate how SelfReference types can be created and 
/// how we need to be careful of this type but the methods we call.
///
/// The reason this is important is because if we were going to use a 
/// async function using Self reference for tokio/threads, we need to 
/// unsure memory is carefully handles. Imagine, if were to pass ownership
/// and some files were copies, but the reference in ptr, points to the old
/// address. For that very reason we try to simulate the methods we will be
/// calling and how they are handled.
#[derive(Debug)]
pub struct MySelfReference {
    // Note: the only way we an access these values are through the methods
    // we will be calling on them.
    val: u8,
    ptr: Option<*const u8>,
}

// copy clone doesnt matter here really
#[derive(Debug, Copy, Clone)]
pub(crate) struct MySelfReferencePinned {
    // Note: the only way we an access these values are through the methods
    // we will be calling on them.
    val: u8,
    ptr: Option<*const u8>,
    // when we use PhantomPinner: the MySelfReference struct goes from
    // Unpin type ( via auto implementations ) to !Unpin type
    _mkr: std::marker::PhantomPinned,
    
    // in the case a type is Unpin, we can use
    // Pin::new(), Box::pin() and pin!().
    // however, if use make the type !Unpin, then this will
    // not work for pin::new() cause pin::new() only works
    // for types that are Unpin.
}

impl MySelfReference {
    fn new(val: u8) -> Self {
        Self { val, ptr: None }
    }
    
    fn put_ptr(&mut self) {
        self.ptr = Some( &raw const self.val );
    }
    
    fn get_val(&self) -> u8 {
        unsafe { *self.ptr.unwrap() }
    }
    
    fn update_val(&mut self, val: u8) {
        self.val = val;
    }
    
    fn update_val_ptr(&mut self, val: u8) {
        *&mut self.val = *&val;
    }
    
    fn print_addr(&self) {
        println!("add of variable is {:p}", &self);
        println!("add of val is {:p}", &raw const self.val);
        println!("add of ptr is {:p}\n", &self.ptr.unwrap());
    }
}

impl MySelfReferencePinned {
    fn new(val: u8) -> Self {
        Self { val, ptr: None, _mkr:std::marker::PhantomPinned }
    }
    
    fn put_ptr(&mut self) {
        self.ptr = Some( &raw const self.val );
    }
    
    fn get_val(&self) -> u8 {
        unsafe { *self.ptr.unwrap() }
    }
    
    fn update_val(&mut self, val: u8) {
        self.val = val;
    }
    
    fn update_val_ptr(&mut self, val: u8) {
        *&mut self.val = *&val;
    }
    
    fn print_addr(&self) {
        println!("add of variable is {:p}", &self);
        println!("add of val is {:p}", &raw const self.val);
        println!("add of ptr is {:p}\n", &self.ptr.unwrap());
    }
}

