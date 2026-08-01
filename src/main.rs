#![allow(dead_code)]
#![allow(unused)]
use std::ops::Deref;
use paste::paste;

#[allow(unused_macros)]
macro_rules! my_func {
    // 'm' is for literal matching.
    // ident can be used here more for alphabets. cant start with a number
    // but ident can have numbers present else where.
    ( 'm', $md:literal, $($va:ident),* ) => {{ $(
            paste! {
                [<mod $md _ $va>]();
            }
        )*
    }};
    ( 't', $nb:literal, "", $va:literal ) => {{
        paste! {
            [<th$nb>]::[<thread $va>]();
        }
    }};
    ( 't', $nb:literal, $nest: literal, $va:literal ) => {{
        paste! {
            [<th $nb>]::[<th $nest>]::[<thread $va>](); }
    }};
}

// this means that files that point to main.rs dont expose the 
// files that point to th1. Its only just for this file unless
// explicitly stated somewhere else.
mod th1;
mod th2;
mod th3;

//todomanish:
use pointers_threads::*;

// I block on mod3_a directly to avoid changin main via tokio::main
// #[tokio::main]
fn main() {
    // my_func!('m', 1, c);
    // These two below are not really useful. But just having fun a bit
    // with how the macros are running.
    // Cause we can just call the function directly when we are testing
    // my_func!('t', 1, "", "1st");
    // my_func!('t', 1, "1a", "1a");
    //
    // my_func!('m', 2, a);
    // my_func!('m', 1, g).await;
    // mod1_b();
    main_test();
    //
    //

    let rt = tokio::runtime::Runtime::new().expect("testing");
    let t = rt.block_on(two());
    println!("t is {t}");
}

async fn one(val: u8) {
    println!("one - {val}");
}

async fn two() -> u8 {
    one(0).await;
    println!("two 1");
    one(1);
    println!("two 2");
    one(2).await;
    println!("two 3");
    one(3);
    println!("two 4");
    one(4);
    println!("two 5");
    one(5).await;
    5
}

fn mod1_a() {
    // I dont need to do anymore: use pointers_threads::lib_th1a::*;
    // Cause I have threads1 mod exposing lib_th1a functions
    // But I still prefer to do it so that I can test based on the module.
    // use threads1::*;
    // use pointers_threads::lib_th_a::*;
    // use pointers_threads::thread1st_get_current;

    let a1 = thread1a_add_42(vec![1, 2, 3, 4, 5]);
    println!("a1 is :{:?}", a1);

    let a2 = thread1a_add_val(a1, 9);
    println!("a2 is :{:?}", a2);

    println!("move issue is three {}", thread1a_move_issue());

    println!("builder: {:?}", thread1a_builder(vec![1, 2, 3, 4, 5], true));

    println!(
        "thread stat owned for \"hello world\" is :{}",
        thread1a_stat_owned("hello world")
    );

    println!("Stat funcion val: {}", thread1a_stat() );

    // This also works
    // println!("val for th scope vec is {:?}", 
    //     thread1a_scope_vec::<std::ops::Range<i32>, std::ops::Range<char>, i32, char>(
    //         (1..1000), ('a'..'j'), 'j', true
    //     )
    // );
    println!("val for th scope vec is {:?}", thread1a_scope_vec((1..1000), ('a'..'j'), 'j', true));

    println!("we got this thread id: {:?}", thread1st_get_current());
}

fn mod1_b() {
    // use pointers_threads::lib_th_b::*;

    println!("leaked value from box leak for thread avg: {}", thread1b_box_leak_avg(1..=10) );
    thread1b_forget_leak();
}


fn main_test() {
    // use pointers_threads::lib_ptr_a::*;
    let mut u = MySelfReference::new(4u8);
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
    
    let mut u = MySelfReference::new(4u8);
    u.put_ptr();
    let mut pu = Box::pin(u);
    pu.get_val(); //  work
    pu.update_val(44); // works
    pu.update_val_ptr(48); // works
    pu.print_addr();
    
    // pined options ------ with pin::new()
    let mut u = MySelfReference::new(4u8);
    u.put_ptr();
    // we do put ptr 1st cause later, after pin_u, we cant
    // as we hand the mut ref to pin_u
    let mut pin_u = std::pin::Pin::new(&mut u);
    pin_u.put_ptr();
    // but we could still do pin_u.put_ptr();
    
    // cant modify directly with u if we are using pin_u
    // after the value. This way, we lock the value.
    // u.val = 8;
    
    //todomanish:
    // pin_u.val = 1; // todomanish, this will only works inside of lib crates
    //                // as the fields are only pub to the crate
    // println!("val pin_u get {:}", pin_u.get_val());
    
    // we have the option to choose here depending on the
    // lifetime of the pin_u or u.
    u.print_addr();
    //pin_u.print_addr();
    
    /*
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
    */
}

fn mod1_c() {
    th1::th1c::testing1c();
}

fn mod1_d() {
    th1::th1d::thread1d_mutex_lock_attempt();
    th1::th1d::thread1d_park_mutex();
    th1::th1d::thread1d_arc_mutex();
}

fn mod1_e() {
    th1::th1e::thread1e_arc_rwlock();
    th1::th1e::thread1e_rwlock_is_finished();
}

fn mod1_f() {
    th1::th1f::thread1f_cow();
}

fn mod1_g() {
    // let x = th1::th1g::state1_oop_non_oop::Post_OOP::new();
    th1::th1g::state1_oop_non_oop::thread1g_state_pattern();
    th1::th1g::state1_oop_non_oop::thread1g_type_pattern();
    th1::th1g::state1_oop_non_oop::thread1g_any_pattern();
}

fn mod2_a() {
    th2::thread2nd();
    println!("-----------------------------MYCELL--------------------------------");
    th2::th2a::testing_mycell();
    println!("-----------------------------MYREFCELL--------------------------------");
    th2::th2a::testing_myrefcell();
    println!("-----------------------------MYRC--------------------------------");
    th2::th2a::testing_myrc_cell();
    // this is how I will be able to get the MYCell and other pointers
    // let a = th2::th2a::MyCell::new(33); // because I used pub use self of MyCell
    // println!("a get is {}", a.get());
}

fn mod2_b() {
    th2::th2b::thread2b_RcWeak();
}

// this also works, but we need to block on the main function with tokio::main. I prefer
// to do it in the mod3_a function which prevent me from writing tokio::main
// async fn mod3_a() {
//     th3::th3a::thread3a_normal_async().await;
//     th3::th3a::thread3a_normal_async().await;
// }
fn mod3_a() {
    let rt = tokio::runtime::Runtime::new().expect("rt for mod3_a");
    rt.block_on({
        println!("1st normal");
        th3::th3a::thread3a_normal_async()
    }); // talks blocked here

    println!("\nNext we have multi");
    rt.block_on(th3::th3a::thread3a_multi_async())
}


fn mod3_b() {
    th3::th3b::thread3b_async_runner();
}

fn mod3_c() {
    //todomanish: Need to just implement for async
    th3::th3c::some_async();
}



// For testing, I have these following
//
// This is my understanding.
// For Lib crate:
// If we will have to have a test module inside the lib create:
// meaning: #[test], or #[cfg(test)], it will allow us to access all
// the structs and fields and methods that are exposed to the create.
// eg: #[cfg(test)] in lib.rs or lib modeule via pub(create) or withing
// the same module ( used for helper, fields and edge cases)
// Can have #[cfg(test)] and #[test]
// NOTE: Only lib can access private items belonging to the lib crate
// or module is path is correct.
//
//
// NOTE: Integration test, meaning creating a test folder, is meant
// to be used for lib create. This will not expose 
// the pub create data types, unless they are completely public.
// Integration tests creates its own create, that is separate from
// lib, doc and binary creates. This depends on the package as a whole.
// You, cannot expose main or any other binary file in integration tests
// because they are not a library.
//
// For: Integration create: 
// Rust creates a separate create for each file inside Integration folder.
// We import lib here ( like how to do from any project ). And we can 
// access the types depending if they are public or not.
// Cant use private fields present for lib create
// Can have #[cfg(test)] and #[test], but we dont need to really specify
// the #[cfg(test)], cause its already in the test integration folder.
// Simple #[test] is enough
// We need to treat integration tests as thought we are an external 
// used using this lib or binary.
//
// For Binary crate:
// In the case for Binary crate, rust creates a spearate crate for each 
// binary file recognised inside the root crate folders for bin. In
// will test this be adding #[cfg(test)] or #[test] directly inside
// on the binary crate. This is how we will manage them.
// Cant use private fields present for lib create. ( has to be only
// those in bin create is in correct module )
// Can have #[cfg(test)] and #[test]
//
// Other crate types: 
// Examples: This is another crate root used to test examples and
// each file is considered as a seprarate crate
// Benches: This is also another crate root used to test benchmarks
// and each file is treated as a seprarate crate
// Build: This is just a single file build.rs that creates one crate
// that can be used to build a project
// Can have #[cfg(test)] and #[test]. Generally we dont need to have
// #[test] or #[cfg(test)] for build, but it still works
