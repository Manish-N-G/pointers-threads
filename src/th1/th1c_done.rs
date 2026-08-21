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
