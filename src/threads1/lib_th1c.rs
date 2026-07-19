//!
//!
//!
//!
//!
//!
//!
//!
//!
//!
//!
//!
//!
//!
//!
//!
pub fn testing1c() {
    println!("testing");
    let a = [1, 2, 3];
    println!("a is {}", unsafe { a.get_unchecked(2) });

    // This works
    // let mut b:u16 = 33;
    // let c = &b as *const u16;
    // let d = &mut b as *mut u16;

    let mut b: u32 = 2151686160; //10000000 01000000 00100000 00010000
    #[allow(unused)]
    let c = &b as *const u32 as *const u16; // 00100000 00010000
    let d = &mut b as *mut u32 as *mut u16;
    //or
    let c = std::ptr::addr_of!(b) as *const u16; // 00100000 00010000 // 8208

    danger_pass(c, d);
}

fn danger_pass(a: *const u16, b: *mut u16) {
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

