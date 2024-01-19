// benches/slice_benchmark.rs
use criterion::{criterion_group, criterion_main, Criterion};
use conway::game_of_life::slice::Slice;

fn benchmark_next_generation(c: &mut Criterion) {
    let mut group = c.benchmark_group("Slice next_generation");
    let mut slice = Slice::new(256);
    slice.randomize();

    group.bench_function("naive", |b| {
        b.iter(|| slice.next_generation_naive())
    });
    group.bench_function("naive_optimized", |b| {
        b.iter(|| slice.next_generation_naive_optimized())
    });
    group.finish();
}

criterion_group!(benches, benchmark_next_generation);
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
