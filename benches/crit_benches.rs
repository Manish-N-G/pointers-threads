use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;
// since crate referes to the root of the current module's create, this
// means we cannot use the create label to get the module. Here, benches
// are not associated to the project create, but it compiled as a separate
// crate, which for us here is to bench our functions
// use crate::lib_th_c::*;
use pointers_threads::lib_th_c::*;

fn criterion_benchmark(c: &mut Criterion) {
    // c.bench_function("fib 20", |b| b.iter(|| fibonacci(black_box(20))));
    c.bench_function("1c lock_attempt", |b| {
        b.iter(|| thread1c_mutex_lock_attempt(false, 65500))
    });
    c.bench_function("1c lock_attempt with black_box", |b| {
        b.iter(|| thread1c_mutex_lock_attempt(false, black_box(65500) ))
        // what does black_box even do?
    });
}

criterion_group!(bb, criterion_benchmark);
criterion_main!(bb);
