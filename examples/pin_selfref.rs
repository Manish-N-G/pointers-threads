use pointers_threads::lib_ptr_a::*;

fn main() {
    // for cargo run --example pin_selfref one
    // args will be only one
    let mut v = std::env::args().skip(1).take(1);
    match v.next() {
        Some(val) if &val == "one" => test_self_ref(),
        Some(val) if &val == "two" => test_self_ref_pin(),
        _ => {
            test_self_ref();
            test_self_ref_pin();
        },
    }
}


fn test_self_ref() {
    println!(" --------------------------- Test Self Ref ----------------------------");
    println!("Normal ======================");
    let mut u = MySelfReference::new(4u8);

    // This assert_val functing will compile
    // fn assert_val<T: Unpin>() {}
    // assert_val::<MySelfReference>();

    u.put_ptr();
    u.print_addr( &format!("Created new SelfRef and put ptr {}", u.get_val()) );
    // this can be dangerous, cause this value gets moved into a new
    // varaiable, and hence, the values will have a new address.
    // Here, the self ref will point to the old one. And we have to be
    // careful to update the values.
    let mut v = u;
    v.print_addr( &format!("Value of ptrs after moved to new variable {}", v.get_val()) );
    
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

    // The only way is to do a put_ptr()
    w.put_ptr();
    w.print_addr( &format!("put ptr must, {}", w.get_val()) );
    

    // ============================== BOX::PIN ============================
    println!("BOX::PIN ======================");
    let mut u = MySelfReference::new(10u8);
    u.put_ptr();
    u.print_addr( &format!("created new var for self ref {}", u.get_val()) );
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
    *pu = MySelfReference::new(34u8); 
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

    pu.update_val(24); // works
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
    let mut u = MySelfReference::new(23u8);
    u.put_ptr();
    u.print_addr("Add for Put Ptr for pin new");
    // we do put ptr 1st cause later, after pin_u, we cant
    // as we hand the mut ref to pin_u
    let mut pin_u = std::pin::Pin::new(&mut u);
    // let mut pin_u = std::pin::Pin::new(&u); // will give borrow error
    pin_u.update_val(10u8);
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

    // ============================== PIN::PIN! ============================
    println!("PIN::PIN! ======================");
    // NOTE: works for Unpin types

    let mut u = MySelfReference::new(12u8);
    u.put_ptr();
    u.print_addr( &format!("Add for Put Ptr before pin pin! {}", u.get_val()) );

    let mut pin_u = std::pin::pin!(&mut u);
    pin_u.update_val(51u8);
    pin_u.print_addr( &format!("before put_ptr after pin new: {}", pin_u.get_val()) );
    //NOTE: Notice that it still works for update_val if we were to chage
    //the value. Workd for Pin!

    pin_u.put_ptr(); // This is not needed to be called anymore
    // u.print_addr("Add After pin new"); // wont works
    // but we could still do pin_u.put_ptr();

    // we have the option to choose here depending on the
    // lifetime of the pin_u or u.
    pin_u.print_addr( &format!("after pinned value via pin new and val is: {}", pin_u.get_val()) );
    // NOTE: also notice that calling put pointer also works here
    // when we print the value
}


fn test_self_ref_pin() {
    println!(" --------------------------- Test Self Ref Pinned----------------------------");
    println!("Normal ======================");
    let mut u = MySelfReferencePinned::new(34u8);
    u.put_ptr(); // dont forget or will panic
    u.print_addr( &format!("put val {}", u.get_val()) );
    u.update_val(40); // works before pinning
    u.print_addr( &format!("update val :{}", u.get_val()) );
    u.update_val_ptr(49); // works before pinning
    u.print_addr( &format!("the value with updata val ptr {}", u.get_val()) );


    let mut v = u;
    v.print_addr( &format!("put val moved{}", v.get_val()) );
    v.update_val(11); // works before pinning
    v.print_addr( &format!("update val moved :{}", v.get_val()) );
    v.update_val_ptr(84); // works before pinning
    v.print_addr( &format!("the value with updata val ptr moved {}", v.get_val()) );

    //NOTE: now this doesnt work. Cause the value is moved and points
    //to the wrong location. We need to update the pointer location
    v.put_ptr(); // dont forget or will panic
    v.print_addr( &format!("put ptr now {}", v.get_val()) );

    
    // ============================== BOX::PIN ============================
    println!("BOX::PIN ======================");
    let mut r = MySelfReferencePinned::new(83u8);
    r.put_ptr(); // this will works
    r.print_addr( &format!("Put ptr with cast as val is {}", r.get_val()) );
    
    let mut _u = MySelfReferencePinned::new(19u8);
    // Now we cant unpin this data.
    // what this means is that we wont be able to run some values
    let pu = Box::pin(_u);
    // pu.print_addr( "box pin for pinned" ); // this will panic. Call ptr on None type
    // pu.put_ptr(); // this will not works
    pu.put_ptr_cast(); // this will works
    pu.print_addr( &format!("Put ptr with cast as val is {}", pu.get_val()) );
    //pu.update_val(44); // cannot borrow data as mutable // doesnt work

    pu.update_val_ptr_cast(94); // cannot borrow data as mutable // doesnt work
    pu.print_addr( &format!("Update ptr cast val {}", u.get_val()) );
    // NOTE: Notice here, when we call update value, it doesnt give us the
    // correct info, as the address points to the old one. This is same
    // as the Update_val function
    //
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


    pu.put_ptr_cast(); // this will works again. and seems to correct the update value
    // Will update the correct address when we call this put ptr cast
    pu.print_addr( &format!("Put ptr with cast again for val {}", pu.get_val()) );
    
    // ============================== PIN::NEW ============================
    println!("PIN::NEW ======================");
    println!("Doesnt work for Pinned types !Unpin\n");
    // let mut _u = MySelfReferencePinned::new(8u8);
    // let pu = std::pin::Pin::new(_u);
    // Pin new will not work for this
    //
    // NOTE: This wont work
    // let mut pin_u = std::pin::Pin::new(&mut u);
    // note: we cannot use pin::new() here
    // PhantomPinned cannot be unpinned

    // ============================== PIN::PIN! ============================
    println!("PIN::PIN! ======================");
    let mut u = MySelfReferencePinned::new(88u8);

    u.put_ptr();
    u.print_addr( &format!("Add for Put Ptr before pin pin! {}", u.get_val()) );

    let pin_u = std::pin::pin!(u);
    pin_u.print_addr( &format!("Put ptr with cast as val is {}", pin_u.get_val()) );

    // pin_u.update_val(5u8); // cannot use this
    pin_u.update_val_ptr_cast(93); // cannot borrow data as mutable // doesnt work
    pin_u.print_addr( &format!("before put_ptr after pin pin!: {}", pin_u.get_val()) );

    // pin_u.put_ptr(); // This will not work.
    pin_u.put_ptr_cast(); // This not needed. But harmless to have

    // we have the option to choose here depending on the
    // lifetime of the pin_u or u.
    pin_u.print_addr( &format!("after pinned value via pin new and val is: {}", pin_u.get_val()) );
    // NOTE: This is very strange. But behind the hood, it makes more sence.
    // Notice that here, put_ptr_cast actually causes a problem and this produecs
    // and error. This implmentation is not the same seem in Pin::new and
    // Box::pin. Behind the hood, pin! create a temp value which then reassigns
    // the result, preventing &mut self to work. However, when we do ptr manuplation
    // directly, this causes error. Its for this reason, I have created this file.
    // I will help describe and process all the different methods we have
    // when we use pinned type for !Unpin and the pin!, new, and Box pin methods
    // for process these values. Its for this reason, we should now be using these
    // states. But its better to use a state pattern which allow us to control the
    // implementation better.
    
}
