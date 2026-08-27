#![cfg_attr( feature = "nightly-bench", feature(test) )]
// means allows feature(test) if feature for nightly-bench is active
// #![feature(test)]

// it wont compile if I dont specify cfg feature "nightly_bench"
// this is because the stable version or rust currently doesnt 
// support feature(true) directly like that. So I have decided to
// add attribute config to specify that this code will compile for
// only feature = nightly-bench
#[cfg( feature = "nightly-bench" )]
extern crate test;
#[cfg( feature = "nightly-bench" )]
use test::Bencher;
#[cfg( feature = "nightly-bench" )]
use test::black_box;

use rayon::prelude::*;
use pointers_threads::lib_th_c::*;

fn get_incremental_sum_from_index_count( idx: u16, count: u64 ) -> u128 {
    ((idx as u128)..=(idx as u128+count as u128)).into_par_iter().sum()
}
 
#[cfg( feature = "nightly-bench" )]
// we just add the tag:
#[bench]
fn normal_bench(b: &mut Bencher) {
    b.iter( || get_incremental_sum_from_index_count( 400, 423293393) );
    b.iter( || thread1c_mutex_lock_attempt(false, 65500) );
    b.iter( || get_incremental_sum_from_index_count( black_box(303), black_box(23423439)) );
    // issues this loop
    b.iter( || thread1c_arc_mutex_display( &["display"], 1000, false) );
    // b.iter( || thread1c_arc_mutex_display( &["loop"], 1000, false) );
}


// black_box here is a function used to prevent the rust compiler from optimizing or
// caching benchmarked computations. this ensures that the benchmaks are done in runtime
// rather then caching and optimizing to gain unfair performance during tests.

// By default, the rust compiler may eliminate code snippits that could better optimize the
// operation for a perticular function. This dynamic calls or perprocessed iterator functions.

// for use, we can just run: 
// cargo +nightly bench --features nightly-bench
// 
// This will not work:
// cargo bench --features nightly-bench
