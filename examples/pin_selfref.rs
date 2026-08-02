use pointers_threads::lib_ptr_a::*;

fn main() {
    // for cargo run --example pin_selfref one
    // args will be only one
    let mut v = std::env::args().skip(1).take(1);
    match v.next() {
        Some(val) if &val == "one" => {
            println!(" --------------------------- Test Self Ref ----------------------------");
            test_self_ref();
        },
        Some(val) if &val == "two" => {
            println!(" --------------------------- Test Self Ref ----------------------------");
            test_self_ref_pin();
        }
        _ => {
            // println!(" --------------------------- Test Self Ref ----------------------------");
            // test_self_ref();
            println!(" --------------------------- Test Self Ref ----------------------------");
            test_self_ref_pin();
        },
    }
}


fn test_self_ref() {
    println!("Normal ======================");
    let mut u = MySelfReference::new(4u8);
    u.put_ptr();
    u.print_addr("Created new SelfRef and put ptr");
    // this can be dangerous, cause this value gets moved into a new
    // varaiable, and hence, the values will have a new address.
    // Here, the self ref will point to the old one. And we have to be
    // careful to update the values.
    let mut v = u;
    v.print_addr("Value of ptrs after moved to new variable");
    
    v.update_val(7u8); // this updates the value. I dont tsee any add change when updates
    v.print_addr( &format!("Value updates: But does it reflect the ptr val {}", v.get_val()) );
    // NOTE: Notice here, when we call update value, it doesnt give us the
    // correct info, as the address points to the old one.
    
    v.put_ptr();
    v.print_addr( &format!("New put ptr after updated value, {}", v.get_val()) );

    let mut w = v;
    w.update_val_ptr(9u8);
    w.print_addr( &format!("Update via val ptr: {}", w.get_val()) );
    // NOTE: Notice here, when we call update value, it doesnt give us the
    // correct info, as the address points to the old one. This is same
    // as the Update_val function
    

    // ============================== BOX::PIN ============================
    println!("BOX::PIN ======================");
    let mut u = MySelfReference::new(4u8);
    u.put_ptr();
    u.print_addr("created new var for self ref");
    let mut pu = Box::pin(u); // this assigns a new address
                              // again as box pin pins the value
                              // to the address. Box pin makes
                              // a heap allocation
    // let val = *pu; // since this is pinned via box pin,
                   // this will not compile. The reason is
                   // this ensures that the value inside the 
                   // box is never moved. Only the pointer 
                   // (box pointer) is okay to move, while the
                   // inner value remains fixed.

    // examining this, we see that MySelfReference does not implement copy
    // This assert will not compile
    // fn assert_copy<T: Copy>() {}
    // assert_copy::<MySelfReference>();
    
    println!("{}\n", std::any::type_name_of_val(&pu));

    pu.print_addr("Here we pin value via Box pin. The important thing to note here is that \n\
        the pointer to the box can change. However it maintains the ptr value will point \n\
        to the old one, so we have to be careful here");

    // NOTE: How is the possible, if I have pinned the value?
    *pu = MySelfReference::new(42u8); 
    // From my understanding, we can deref only Unpin types
    // here. This should not work for !Unpin types?
    // Perhaps assignement is not the same as moving out
    // the value. Hence we should be able to do this if we
    // are creating a new type in the same address location.
    //
    // let v: Vec<std::pin::Pin<Box<dyn Unpin>>> = vec![pu]; // this works

    pu.put_ptr();
    pu.print_addr( &format!("reuploaded the put ptr: {}", pu.get_val()) );

    println!("We use update_val here");
    pu.update_val(44); // works
    pu.print_addr( &format!("update val gives {}", pu.get_val()) );
    // NOTE: Notice here, this implementation gives us a different result
    // compared to what we have earlier. Now that the value in pinned via
    // Box Pin, when we upload the value, it always points to the correct
    // address. And we get the correct value.

    println!("update via val ptr");
    pu.update_val_ptr(48); // works
    // NOTE: looks like update_val_ptr doesnt make a difference
    println!("updated val {}", pu.get_val());
    pu.print_addr("");

    pu.put_ptr();
    println!("updated val via put ptr is not needed: Val {}", pu.get_val());
    pu.print_addr("");
    
    // ============================== PIN::NEW ============================
    println!("PIN::NEW ======================");
    let mut u = MySelfReference::new(4u8);
    u.put_ptr();
    u.print_addr("Add for Put Ptr for pin new");
    // we do put ptr 1st cause later, after pin_u, we cant
    // as we hand the mut ref to pin_u
    let mut pin_u = std::pin::Pin::new(&mut u);
    pin_u.update_val(5u8);
    pin_u.print_addr( &format!("before put_ptr after pin new: {}", pin_u.get_val()) );
    //NOTE: Notice that it still works for update_val if we were to chage
    //the value.
    pin_u.put_ptr(); // This is not needed to be called anymore
    // u.print_addr("Add After pin new"); // wont works
    // but we could still do pin_u.put_ptr();
    
    // cant modify directly with u if we are using pin_u
    // after the value. This way, we lock the value.
    // u.val = 8;
    
    //todomanish: We will create unit test in lib for this
    // pin_u.val = 1; // todomanish, this will only works inside of lib crates
    //                // as the fields are only pub to the crate
    // println!("val pin_u get {:}", pin_u.get_val());
    
    // we have the option to choose here depending on the
    // lifetime of the pin_u or u.
    pin_u.print_addr( &format!("after pinned value via pin new and val is: {}", pin_u.get_val()) );
    //pin_u.print_addr();
}


fn test_self_ref_pin() {
    println!("Normal ======================");
    let mut u = MySelfReferencePinned::new(4u8);
    u.put_ptr(); // dont forget or will panic
    u.print_addr( "put val" );
    u.update_val(44); // works before pinning
    u.print_addr( &format!("update val :{}", u.get_val()) );
    u.update_val_ptr(49); // works before pinning
    u.print_addr( &format!("the value with updata val ptr {}", u.get_val()) );

    // note: we cannot use pin::new() here
    // PhantomPinned cannot be unpinned
    // NOTE: This wont work
    // let mut pin_u = std::pin::Pin::new(&mut u);
    
    let mut _u = MySelfReferencePinned::new(8u8);
    // Now we cant unpin this data.
    // what this means is that we wont be able to run some values
    let pu = Box::pin(_u);
    pu.print_addr( "box pin for pinned" );
    // pu.put_ptr(); // this will not works
    pu.put_ptr_cast(); // this will works
    u.print_addr( "hello" );
    pu.print_addr( &format!("Put ptr with cast as val is {}", pu.get_val()) );
    //pu.update_val(44); // cannot borrow data as mutable // doesnt work
    pu.update_val_ptr_cast(44); // cannot borrow data as mutable // doesnt work
    u.print_addr( &format!("Update add cast {}", u.get_val()) );
    // pu.update_val_ptr(48); // cannot borrow data as mutable // doesnt work
    // NOTE: In principly, for pinned marker, making the trait for this
    // type !Unpin, rust doest let us to pu.getval = something or 
    // the same for (*pu).get_val
    // This is because Pin intentially hides the value for
    // MySelfReferencePinned or type !Unpin.
    // The reason is beause we should not be able to do something
    // like std::mem::replace or *r = another.
    // Hence Our solution need to be modified.
    pu.print_addr("");
    
    
}
