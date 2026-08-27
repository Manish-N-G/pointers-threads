// for use, we can just run: 
// cargo bench
use criterion::{Criterion, criterion_group, criterion_main};
use criterion::{BenchmarkId};
use std::hint::black_box;
// since crate referes to the root of the current module's create, this
// means we cannot use the create label to get the module. Here, benches
// are not associated to the project create, but it compiled as a separate
// crate, which for us here is to bench our functions
// use crate::lib_th_c::*;
use pointers_threads::lib_th_c::*;

fn bench_lock(crit: &mut Criterion) {
    // c.bench_function("fib 20", |b| b.iter(|| fibonacci(black_box(20))));
    crit.bench_function("1c lock_attempt", |b| {
        b.iter(|| thread1c_mutex_lock_attempt(false, 65500))
    });
    crit.bench_function("1c lock_attempt with black_box", |b| {
        b.iter(|| thread1c_mutex_lock_attempt(false, black_box(65500) ))
        // what does black_box even do?
        // black_box here is a function used to prevent the rust compiler from optimizing or
        // caching benchmarked computations. this ensures that the benchmaks are done in runtime
        // rather then caching and optimizing to gain unfair performance during tests.

        // By default, the rust compiler may eliminate code snippits that could better optimize the
        // operation for a perticular function. This dynamic calls or perprocessed iterator functions.
    });
}

criterion_group!(locks, bench_lock);
// criterion_main!(locks); // we call locks later again

// ---------------------------------------------------------------

fn bench_lock_with_sample_size(crit: &mut Criterion) {
    let mut group = crit.benchmark_group("arc_locks");

    // Configure Criterion.rs to detect smaller differences and increase sample size to improve
    // precision and counteract the resulting noise.
    group.significance_level(0.1).sample_size(50);

    // NOTE: if we all too must, it will definitely cause problems
    for i in [5u16, 65500].iter() {
        group.bench_with_input(BenchmarkId::new("lock 1", i), i, 
            |b, i| b.iter(|| thread1c_mutex_lock_attempt( false, *i )));
        group.bench_with_input(BenchmarkId::new("lock 2", i), i, 
            |b, i| b.iter(|| thread1c_mutex_lock_attempt( false, black_box(*i) )));
    }
    group.finish();
}

criterion_group!(locks_samples, bench_lock_with_sample_size);


// ---------------------------------------------------------------

// NOTE: Not working for arc_mutex_display currently
fn bench_arc_display(crit: &mut Criterion) {
    let mut group = crit.benchmark_group("arc_locks");

    // Configure Criterion.rs to detect smaller differences and increase sample size to improve
    // precision and counteract the resulting noise.
    group.significance_level(0.1).sample_size(10);

    // NOTE: if we all too must, it will definitely cause problems
    for i in [1u64, 10].iter() {
        // group.bench_with_input(BenchmarkId::new("loop rayon1", i), i, 
        //     |b, i| b.iter(|| thread1c_arc_mutex_display( &["loop -1"], *i, false )));
        // group.bench_with_input(BenchmarkId::new("loop rayon2", i), i, 
        //     |b, i| b.iter(|| thread1c_arc_mutex_display( &["loop -2"], *i, false )));
        // group.bench_with_input(BenchmarkId::new("loop rayon3", i), i, 
        //     |b, i| b.iter(|| thread1c_arc_mutex_display( &["loop -3"], *i, false )));
        // group.bench_with_input(BenchmarkId::new("loop normal", i), i, 
        //     |b, i| b.iter(|| thread1c_arc_mutex_display( &["loop"], *i, false )));
        group.bench_with_input(BenchmarkId::new("display", i), i, 
            |b, i| b.iter(|| thread1c_arc_mutex_display( &["display"], *i, false )));
    }

    group.finish();
}

criterion_group!(display_loop, bench_arc_display);
// criterion_main!(locks, display_loop);
criterion_main!(locks, locks_samples); //works
// criterion_main!(locks, locks_samples, display_loop); // issue with bench_arch_display


