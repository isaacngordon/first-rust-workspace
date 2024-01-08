mod game_of_life;
// use std::thread;
// use std::time::Duration;

fn main() {
    const n : usize = 4;
    // const max_fps : f64 = 60.0;

    let mut game = game_of_life::Slice::new(n);
    game.randomize();
    println!("{}", game);

    let start = std::time::Instant::now();
    let mut last_frame_time = std::time::Instant::now();
    
    for i in 0..10000 {
        game.next_generation_naive();

        // Clear the console
        print!("\x1B[2J\x1B[1;1H");

        let fps = 1.0 / last_frame_time.elapsed().as_secs_f64();
        println!("\n+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=\n\t ==> Step {}   FPS: {}\n\n", i, fps);
        println!("{}", game.to_hex_string());
        last_frame_time = std::time::Instant::now();

        // let sleep_time = Duration::from_secs_f64(1.0 / max_fps - last_frame_time.elapsed().as_secs_f64());
        // thread::sleep(sleep_time);
    }
    
    let elapsed = start.elapsed();
    println!("+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=\nElapsed: {}s", elapsed.as_secs_f64());
}