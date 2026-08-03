//! Testing ptr_a
//!
//!
//!

//pub unsafe fn unsafe_raw_vector_element_mutability(vec: Vec<usize>) -> (u16, u16) {
/// An unsafe function that requires a vector of more then
/// 2 elements, or else it would fail. It produces a tuple
/// where the 1st element it the always 8208 and the next is
/// 8209 as long as no wrapping is does if out of bound or 
/// panic is vector it not larger than 2 elements.
/// ```rust,editable
/// //use pointers_threads::lib_ptr_a::*;
/// use pointers_threads::unsafe_raw_vector_element_mutability;
/// // use my_crate::assert_panic_message;
/// let a = vec![1usize, 2, 3];
///
/// // we have to be careful here when we pass the vec.
/// // this function is inherently made so that this will
/// // panic if the vec is less then 3 elements longs.
///
/// // However this function doesnt focus too much
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

    element = 2151686160;
    //10000000 01000000 00100000 00010000
    #[allow(unused)]
    // we convert this to &u16 from u32
    let const_u16 = &element as *const u32 as *const u16;
    // 00100000 00010000

    let mut_u16 = &mut element as *mut u32 as *mut u16;
    //or
    let const_u16 = std::ptr::addr_of!(element) as *const u16;
    // 00100000 00010000 // 8208

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
/// // We could also do
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


#[derive(Debug)]
/// This struct shows a SelfReference type that has a value and a pointers.
/// The methods on this struct call on creating a Self Reference where the 
/// Option of the raw pointers points the value in this struct.
///
/// We use this to illustrate how SelfReference types can be created and 
/// how we need to be careful of this type by the methods we call.
///
/// The reason this is important, is because if we were going to use a 
/// async function using Self reference for tokio/threads, we need to 
/// make sure memory is carefully handles. Imagine, if were to pass ownership
/// and some files were copies, but the reference in ptr, points to the old
/// address. For that very reason we try to simulate the methods we will be
/// calling and how they are handled.
///
/// These types are also affected by how the Box::pin, Pin::new and pin! 
/// operations work. We will have to dive deeper into how those operate,
/// and they to will be discussed in the next part.
/// NOTE: todomanish.
pub struct MySelfReference {
    // Note: the only way we an access these values are through the methods
    // we will be calling on them.
    // Also, by calling a field pub(crate) which is available to the
    // lib only internally, not externally, we can restrict some access
    // Here, we will not use it, but its worth knowing how its done
    pub(crate) val: u8,
    // note: raw pointers dont automatically implement Send and Sync.
    // Hence out file here makes out type not Send and Sync.
    pub(crate) ptr: Option<*const u8>,
}

// Copy clone done seem to matter here really
#[derive(Debug, Copy, Clone)]
/// This SelfReferencePinned type that has a value and a pointers and
/// a marker Pinner Phantom type. We use methods on this type to set the
/// ptr value. We still use the Option of raw pointers to set the inner
/// address
///
/// The purpose of using the SelfReferencePinned type is similar to the
/// SelfReference type. We can access these values through the methods,
/// However, we will soon discover why using PhantomPinned to make this 
/// type a !Unpin, proves to be tricky.
///
/// Here, how this works with async is that, it defines the type to be
/// !Unpin. And if its unpinned, it indicates that it should be used 
/// with mainly Box::pin and not the pin::new and pin! operations. Despite
/// finding ways to implement this for pin::new and pin! safely but working
/// with some workaround, it best to avoid this. Self reference are dangerous
/// if not handles correctly. And we want to limit the possibility of
/// creating more error with using them in async operations.
/// To illustrate: We make this !Unpin type, cause we just want to tell the
/// compiler that this value should not be moved. Its fields will should
/// always hold the same address.
pub struct MySelfReferencePinned {
    // Note: the only way we an access these values are through the methods
    // we will be calling on them.
    pub(crate) val: u8,

    // Also, this field is not Send and not Sync because rust 
    // doesnt automatically make them so as we have used raw pointers.
    pub(crate) ptr: Option<*const u8>,

    // when we use PhantomPinner: the MySelfReference struct goes from
    // Unpin type ( via auto implementations ) to !Unpin type
    _mkr: std::marker::PhantomPinned,
    
    // in the case a type is Unpin, we can use Pin::new(), Box::pin()
    // and pin!(). and those methods works in general.
    // However, if use make the type !Unpin, then this will not work 
    // properly for pin::new() cause pin::new() as this doesnt work with
    // !Unpin. And despite working with pin!(), we also that that this can
    // be really tricky, and we need to be careful about this.
}

/// For this impl block, the methods exposed to it are generally
/// safe. They still use some unsafe code, however, we should be able to
/// manage them. There can still be risk with using async/threads for this
/// type, if we are not careful. Hence, we have to find the correct practice
/// in order to make sure that works well
/// ```
/// use pointers_threads::lib_ptr_a::*; 
///
/// let mut my_self_ref = MySelfReference::new(3u8);
///
/// // To unsure the pointers point to the correct
/// // address in memory
/// my_self_ref.put_ptr();
///
/// assert_eq!(3u8, my_self_ref.get_val());
/// ```
impl MySelfReference {
    /// This simple block to create a new MySelfReference type
    pub fn new(val: u8) -> Self {
        Self { val, ptr: None }
    }
    
    /// This ensures that we have saved the pointer value in ptr
    /// so that I points to the correct value in memory. If we dont
    /// do this, and we more or reassign out variable with a new
    /// MySelfReference type, then the value will show that it
    /// doesnt reflect with the one we want.
    /// ```should_panic
    /// use pointers_threads::lib_ptr_a::*;
    ///
    /// let mut my_self_ref = MySelfReference::new(3u8);
    ///
    /// // if we forget to use put_ptr()
    ///
    /// let (val, ptr) = my_self_ref.get_addresses();
    ///
    /// assert_eq!( val, ptr );
    /// ```
    ///
    /// This would be the correct implementation to make sure
    /// we have put_ptr in our code
    /// ```
    /// use pointers_threads::lib_ptr_a::*;
    ///
    /// let mut my_self_ref = MySelfReference::new(3u8);
    ///
    /// // this put_ptr show now point to the correct memory
    /// my_self_ref.put_ptr();
    ///
    /// let (val, ptr) = my_self_ref.get_addresses();
    ///
    /// assert_eq!( val, ptr );
    ///
    /// ```
    pub fn put_ptr(&mut self) {
        // we use raw pointers to case value to raw address ptr
        self.ptr = Some( &raw const self.val );
    }
    
    /// This get the value VIA POINTERS for our type. We do this 
    /// to show how important it is to handle self references. 
    /// If we are not careful, this will lead to incorrect results
    /// ```should_panic
    /// use pointers_threads::lib_ptr_a::*;
    ///
    /// let mut my_self_ref = MySelfReference::new(3u8);
    ///
    /// // We should not use get_val() without using put_ptr here.
    /// // this is because, get_val relies on the ptr field to get
    /// // raw pointer
    /// my_self_ref.get_val();
    ///
    /// ```
    /// Instead, the correct method should be the one mentioned 
    /// below. We need to always make sure that the value is updated
    /// 1st before getting the value. Or else it could point to an
    /// older value in the address.
    ///
    /// ```
    /// use pointers_threads::lib_ptr_a::*;
    ///
    /// let mut my_self_ref = MySelfReference::new(3u8);
    ///
    /// // we 1st update the value for the raw pointers
    /// my_self_ref.put_ptr();
    ///
    /// let (val, ptr) = my_self_ref.get_addresses();
    /// assert_eq!( val, ptr );
    ///
    /// assert_eq!( 3u8, my_self_ref.get_val() );
    /// ```
    pub fn get_val(&self) -> u8 {
        unsafe { *self.ptr.unwrap() }
    }
    
    /// update_val is used to take an already created value for
    /// MySelfReference, and put a new value in its place for the 
    /// val field. This uses the copy trait and is useful to assign
    /// value quickly without working about the ptr. Here, even it
    /// the ptr is not assigned, we could still assign it
    /// # Warning
    /// Despite being able to update_val, we still have to proceed with
    /// caution.
    /// ```
    /// use pointers_threads::lib_ptr_a::*;
    ///
    /// let mut my_self_ref = MySelfReference::new(3u8);
    ///
    /// // we can update value before saving the ptr. This is 
    /// // possible but not recommended.
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
    pub fn update_val(&mut self, val: u8) {
        self.val = val;
    }
    
    /// We use the update_val_ptr call to update the value in
    /// the field via pointer dereferencing. This essentially is the
    /// came as update_val, but tells us how to use pointers instead.
    /// # Warning
    /// Like Update_val, we still have to proceed with caution, as we
    /// dont want to have calls before ptr is updates. We could still
    /// do it, but not adviced to do so.
    /// ```
    /// use pointers_threads::lib_ptr_a::*;
    ///
    /// let mut my_self_ref = MySelfReference::new(3u8);
    ///
    /// // we can update value before saving the ptr. This is 
    /// // possible but not recommended.
    /// my_self_ref.update_val_ptr(8u8);
    /// ```
    /// Cause we can cause a panic in we are not carefult
    /// ```should_panic
    /// use pointers_threads::lib_ptr_a::*;
    ///
    /// let mut my_self_ref = MySelfReference::new(3u8);
    ///
    /// // we can update value before saving the ptr. This is 
    /// // possible but not recommended.
    /// my_self_ref.update_val_ptr(8u8);
    /// my_self_ref.get_val();
    /// ```
    /// This has a similar to show how the operation is processed.
    /// ```
    /// use pointers_threads::lib_ptr_a::*;
    ///
    /// let mut my_self_ref = MySelfReference::new(3u8);
    /// // Dont forget to use put_ptr 1st
    /// my_self_ref.put_ptr();
    /// let (val, ptr) = my_self_ref.get_addresses();
    /// assert_eq!( val, ptr );
    ///
    /// my_self_ref.update_val_ptr(8u8);
    /// let (val, ptr) = my_self_ref.get_addresses();
    /// assert_eq!( val, ptr );
    ///
    /// my_self_ref.update_val_ptr(18u8);
    /// let (val, ptr) = my_self_ref.get_addresses();
    /// assert_eq!( val, ptr );
    ///
    /// ```
    // does the same as undate_val
    #[allow(clippy::deref_addrof)]
    pub fn update_val_ptr(&mut self, val: u8) {
        *&mut self.val = *&val;
    }

    /// To get the addresses for the value and the ptr raw
    /// address ptr. This is unsafe, and we have to proceed
    /// with caution
    /// ```
    /// use pointers_threads::lib_ptr_a::*;
    ///
    /// let mut my_self_ref = MySelfReference::new(3u8);
    /// my_self_ref.put_ptr();
    /// let (val, ptr) = my_self_ref.get_addresses();
    /// assert_eq!( val, ptr );
    /// ```
    /// The following will panic if we dont set the ptr 1st.
    /// ```should_panic
    /// use pointers_threads::lib_ptr_a::*;
    ///
    /// let mut my_self_ref = MySelfReference::new(3u8);
    /// let (val, ptr) = my_self_ref.get_addresses();
    /// assert_eq!( val, ptr );
    /// ```
    pub fn get_addresses(&self) -> (&u8, &u8) {
        ( &self.val, unsafe { &*self.ptr.unwrap() } )
    }
    
    /// Just a helper function to print values to the
    /// user can see. This can again panic if we dont
    /// setup the ptr interior raw pointer value.
    pub fn print_addr(&self, condition: &str) {
        if !condition.is_empty() {
            println!("{condition}");
        }
        println!("add of variable is {:p}", &self);
        println!("add of val is {:p}", &raw const self.val);
        // careful to not put & here in self, or it will be a 
        // different address value
        println!("add of ptr val is {:p}\n", self.ptr.unwrap() );
    }
}

/// NOTE: Despite being available for Send, we have to proceed
/// with caution.
unsafe impl Send for MySelfReference {}
/// NOTE: Despite being available for Sync, we have to proceed
/// with caution.
unsafe impl Sync for MySelfReference {}

/// I will have to do some unsafe impl here.
/// NOTE: This havs to be to type &self not &mut self. 
/// Or else PhantomPinned marker will complain and not
/// allow you to do this. This is done specifically for
/// safety. Our implementation is unsafe and this should 
/// not be the way this is done. But its good know that 
/// we could find a work aroudn it.. However, dont
/// implement is this way.
/// ```
/// use pointers_threads::lib_ptr_a::*; 
///
/// let mut my_self_ref = MySelfReferencePinned::new(3u8);
///
/// my_self_ref.put_ptr();
///
/// assert_eq!(3u8, my_self_ref.get_val());
/// ```
/// Also, for this type, since we have PhantomPinned type, we should 
/// avoid using ptr manipulation in order to access values.
/// **NOTE: These types are only meant to be used with Box::pin
/// but for the sake of understanding why it could cause problems,
/// we will also look at pin::new ( even if its not possible at the start)
/// at pin! macro.**
impl MySelfReferencePinned {

    /// This simple block to create a new MySelfReferencePinned type
    /// This type has PhantomPinned, so turning this struct from a
    /// Unpin to !Unpin type. !Unpin tells us that this value
    /// has its fields addresses constant, and locked in place
    /// unless we use unsafe rust code.
    pub fn new(val: u8) -> Self {
        Self { val,ptr: None, _mkr:std::marker::PhantomPinned }
    }


    // Notice I used &raw const and not as *const u8
    // cause this will just convert the value into a 
    // addr pointer.
    // starthere
    pub fn put_ptr(&mut self) {
        self.ptr = Some( &raw const self.val );
    }

    // this is wrong. will convert 4 as a pointer value
    // pub fn put_ptr(&mut self) {
    //     // self.ptr = Some( &raw const *self.val.get_mut() ); // works too
    //     self.ptr = Some( self.val as *const u8 );
    // }

    // remember, the add for self is not the same as the address
    // values calling the function. This is because, the address
    // is copied into this, and we use this copied address. However, 
    // the address will point the correct value
    // All this is is a variable holding the address, and this
    // variable is copied.
    pub fn put_ptr_cast(&self) {
        let _z = Some( &raw const self.val );
        // todomanish: Perhaps this is still not correct?
        // even it it works for box::pin
        let mut y = &self.ptr as *const Option<*const u8> as *mut Option<*const u8>;
        unsafe { *y = _z; }

        // didnt work
        // let x = &self.val as *const u8;
        // let mut _y = self.ptr.unwrap();
        // _y = x;
    }

    /// We get the value via ptr from the 
    pub fn get_val(&self) -> u8 {
        // dont use casting for this, will produce the wrong value
        // let x = self.ptr.unwrap() as u32 as *const u32;
        unsafe { *self.ptr.unwrap() }
    }


    pub fn update_val(&mut self, val: u8) {
        self.val = val;
    }
    
    /// # Safety
    /// We this will be able to get the value
    /// between val, and pass the address that way, and deref
    /// that value. This in not recommended to process the values
    /// this way, however, for implementation, it gives uf a 
    /// brief of how it works.
    /// Consider this as unsafe
    pub unsafe fn update_val_cast(&self, val: u8) {
        let mut _x = &self.val as *const u8 as *mut u8;
        unsafe { *_x = val; }
    }
    
    // does the same as undate_val
    #[allow(clippy::deref_addrof)]
    pub fn update_val_ptr(&mut self, val: u8) {
        *&mut self.val = *&val;
    }

    /// Here we are tyring to update the value in val by
    /// using the address what is in the ptr field. This is
    /// a hack, but we could do it this way. An in practice,
    /// this is not the best way to do this in there are
    /// problems with the implementation.
    #[allow(clippy::deref_addrof)]
    pub fn update_val_ptr_cast(&self, val: u8) {
        // let mut x = *self.ptr.unwrap() as *mut u8 ;
        // let mut _x = self.ptr.unwrap() as *mut u8;

        let mut _x = self.ptr.unwrap() as *mut u8;
        unsafe {*_x = val; }
    }

    pub fn print_addr(&self, condition: &str) {
        if !condition.is_empty() {
            println!("{condition}");
        }
        println!("add of variable is {:p}", &self);
        println!("add of val is {:p}", &raw const self.val);
        // careful to not put & here in self, or it will be a 
        // different address value
        println!("add of ptr is {:p}\n", self.ptr.unwrap());
    }
}

