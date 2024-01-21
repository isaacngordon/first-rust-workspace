// benches/slice_benchmark.rs
use criterion::{criterion_group, criterion_main, Criterion};
use conway::game_of_life::slice::Slice;


/// Benchmark the next_generation method for a given size and number of steps using each algorithm
fn benchmark_next_generation(c: &mut Criterion, size: usize, steps: usize) {
    let mut group = c.benchmark_group("Slice next_generation");
    let mut slice = Slice::new(size);
    slice.randomize();

    group.bench_function(format!("naive_{}_{}", size, steps), |b| {
        let mut slice_clone = slice.clone();
        b.iter(|| 
            for _ in 0..steps {
                slice_clone.next_generation_naive()
            }
        )
    });
    group.bench_function(format!("naive_optimized_{}_{}", size, steps), |b| {
        let mut slice_clone = slice.clone();
        b.iter(|| 
            for _ in 0..steps {
                slice_clone.next_generation_naive_optimized()
            }
        )
    });
    group.finish();
}

/// Benchmark each next_generation algorithm for a single iteration for 11 different sizes
fn benchmark_next_generation_single_iteration(c: &mut Criterion) {
    let test_cases = vec![3, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048];
    test_cases.iter().for_each(|size| {
        benchmark_next_generation(c, *size, 1);
    });
}

criterion_group!(benches, benchmark_next_generation_single_iteration);
criterion_main!(benches);


// fn reach_stable_state(slice: &mut Slice) -> usize {
//     let mut count = 0;
//     loop {
//         let prev_slice = slice.clone();
//         slice.next_generation_naive();
//         count += 1;

//         if *slice == prev_slice {
//             break;
//         }
//     }
//     count
// }

// fn benchmark_reach_stable_state(c: &mut Criterion) {
//     c.bench_function("reach_stable_state", |b| {
//         let mut slice = Slice::new(10);
//         slice.randomize(); // Assuming this method exists and sets up an initial state

//         b.iter(|| reach_stable_state(&mut slice))
//     });
// }

// criterion_group!(benches, benchmark_reach_stable_state);
// criterion_main!(benches);
