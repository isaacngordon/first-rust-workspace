use conway::game_of_life;

fn print_result(name: &str, elapsed: std::time::Duration, hex_string: &str) {
    println!("Time elapsed for {}: {}s", name, elapsed.as_secs_f64());
    println!("Hex string: {}", hex_string);
}

fn main() {
    const N: usize = 4;
    let mut game = game_of_life::Slice::new(N);
    game.randomize();

    let mut game_naive = game.clone();
    let mut game_optimized = game.clone();

    // println!("Initial Game: {}", game.to_hex_string());
    // println!("Initial Game Naive: {}", game_naive.to_hex_string());
    // println!("Initial Game Naive Optimized: {}", game_optimized.to_hex_string());

    println!(
        "Naive Start: {}\n{}",
        game_naive.to_hex_string(),
        game_naive
    );
    println!(
        "Optimized Start: {}\n{}",
        game_optimized.to_hex_string(),
        game_optimized
    );

    for i in 0..10 {
        println!("Frame: {} ", i);
        game_naive.next_generation_naive();
        println!("Naive: {}\n{}", game_naive.to_hex_string(), game_naive);
        game_optimized.next_generation_naive_optimized();
        println!(
            "Optimized: {}\n{}",
            game_optimized.to_hex_string(),
            game_optimized
        );
    }

    // let start_naive = std::time::Instant::now();
    // for _ in 0..10000 {
    //     game_naive.next_generation_naive();
    //     // println!("Naive: {}", zgame_naive);
    // }
    // let elapsed_naive = start_naive.elapsed();

    // let start_optimized = std::time::Instant::now();
    // for _ in 0..10000 {
    //     game_optimized.next_generation_naive_optimized();
    //     // println!("Optimized: {}", game_optimized);
    // }
    // let elapsed_optimized = start_optimized.elapsed();

    // print_result("Naive", elapsed_naive, &game_naive.to_hex_string());
    // print_result("Naive Optimized", elapsed_optimized, &game_optimized.to_hex_string());
}
