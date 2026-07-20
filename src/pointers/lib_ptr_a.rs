//! Testing ptr_a
//!
//!
//!

/// testing this
/// rust,editable ??
/// ```
/// use pointers_threads::lib_ptr_a::*;
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
/// rust,editable ??
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

fn main_test() {
    let mut u = MyS::new(4u8);
    u.put_ptr();
    u.print_addr();
    // this is dangerous, cause this is moved into a new
    // varaiable, and hence, the values will have a new
    // address
    let mut v = u;
    v.print_addr();
    
    v.update_val(7u8);
    v.print_addr();
    
    println!("updated val {}", v.get_val());
    v.put_ptr();
    v.print_addr();
    v.update_val_ptr(9u8);
    v.print_addr();
    
    let mut u = MyS::new(4u8);
    u.put_ptr();
    let mut pu = Box::pin(u);
    pu.get_val(); //  work
    pu.update_val(44); // works
    pu.update_val_ptr(48); // works
    pu.print_addr();
    
    // pined options ------ with pin::new()
    let mut u = MyS::new(4u8);
    u.put_ptr();
    // we do put ptr 1st cause later, after pin_u, we cant
    // as we hand the mut ref to pin_u
    let mut pin_u = std::pin::Pin::new(&mut u);
    pin_u.put_ptr();
    // but we could still do pin_u.put_ptr();
    
    // cant modify directly with u if we are using pin_u
    // after the value. This way, we lock the value.
    // u.val = 8;
    
    pin_u.val = 1;
    println!("val pin_u get {:}", pin_u.get_val());
    
    // we have the option to choose here depending on the
    // lifetime of the pin_u or u.
    u.print_addr();
    //pin_u.print_addr();
    
    // box pin options --- box::pin()
    let mut u = MyS2::new(4u8);
    u.put_ptr();
    // note: we cannot use pin::new() here
    // PhantomPinned cannot be unpinned
    // let mut pin_u = std::pin::Pin::new(&mut u);
    
    // Now we cant unpin this data.
    // what this means is that we wont be able to run some values
    let pu = Box::pin(u);
    pu.get_val(); // works
    // pu.update_val(44); // cannot borrow data as mutable // doesnt work
    // pu.update_val_ptr(48); // cannot borrow data as mutable // doesnt work
    pu.print_addr();
    
    
    // box pin options --- box::pin()
    let mut u = MyS2::new(4u8);
    u.put_ptr();
    // note: we cannot use pin::new() here
    // PhantomPinned cannot be unpinned
    // let mut pin_u = std::pin::Pin::new(&mut u);
    
    // Now we cant unpin this data.
    // what this means is that we wont be able to run some values
    let pu = Box::pin(u);
    pu.get_val(); // works
    // pu.update_val(44); // cannot borrow data as mutable // doesnt work
    // pu.update_val_ptr(48); // cannot borrow data as mutable // doesnt work
    pu.print_addr();
    
}


// copy clone doesnt matter here really
#[derive(Debug)]
struct MyS {
    val: u8,
    ptr: Option<*const u8>,
}

// copy clone doesnt matter here really
#[derive(Debug, Copy, Clone)]
struct MyS2 {
    val: u8,
    ptr: Option<*const u8>,
    // when we use PhantomPinner: the MyS struct goes from
    // Unpin type ( via auto implementations ) to !Unpin type
    _mkr: std::marker::PhantomPinned,
    
    // in the case a type is Unpin, we can use
    // Pin::new(), Box::pin() and pin!().
    // however, if use make the type !Unpin, then this will
    // not work for pin::new() cause pin::new() only works
    // for types that are Unpin.
}

impl MyS {
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

impl MyS2 {
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

