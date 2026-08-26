#![cfg_attr( feature = "nightly-bench", feature(test) )]
// #![feature(test)]

// it wont compile if i dont specify cfg feature "nightly_bench"
#[cfg( feature = "nightly-bench" )]
extern crate test;
#[cfg( feature = "nightly-bench" )]
use test::Bencher;
#[cfg( feature = "nightly-bench" )]
use test::black_box;

use rayon::prelude::*;

// todomanish: not sure
// notice, I dont need to do use pointers_threads::lib_th_c::*; 
// this is because, we know that #! recodnizes that this belongs to the
// test bench for our crete
fn get_incremental_sum_from_index_count( idx: u16, count: u64 ) -> u128 {
    ((idx as u128)..=(idx as u128+count as u128)).into_par_iter().sum()
}
 
#[cfg( feature = "nightly-bench" )]
// we just add the tag:
#[bench]
fn normal_bench(b: &mut Bencher) {
    b.iter( || get_incremental_sum_from_index_count( 400, 423293393) );
    b.iter( || get_incremental_sum_from_index_count( black_box(303), black_box(23423439)) );
}

// for use, we can just run: 
// cargo +nightly bench --features nightly-bench
