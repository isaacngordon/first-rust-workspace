use conway::{
    game_of_life::{GlobalDefaults, CameraPlugin, plugin::GameOfLifePlugin}, 
    ui::MainMenuPlugin
};
use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

const DEFAULT_WINDOW_WIDTH: f32 = 1000.0;
const DEFAULT_WINDOW_HEIGHT: f32 = 700.0;

const DEFAULT_GAME_SIZE: usize = 1000;
const DEFAULT_CONTINUOUS_FRAME_RATE: f32 = 5.0;

fn main() {
    App::new()
        .insert_resource(GlobalDefaults {
            window_width: DEFAULT_WINDOW_WIDTH,
            window_height: DEFAULT_WINDOW_HEIGHT,
            game_size: DEFAULT_GAME_SIZE,
            continuous_frame_rate: DEFAULT_CONTINUOUS_FRAME_RATE,
            game_buffer_size: 100,
        })
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Conway's Game of Life".into(),
                resolution: (DEFAULT_WINDOW_WIDTH, DEFAULT_WINDOW_HEIGHT).into(),
                // Add more window settings here as needed...
                ..default()
            }),
            ..default()
        }))
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(CameraPlugin)
        .add_plugins(GameOfLifePlugin)
        .add_plugins(MainMenuPlugin)
        .run();
}


//
// IGNORE BELOW THIS LINE
// this is just a main test function for the game of life
//
//
// fn print_result(name: &str, elapsed: std::time::Duration, hex_string: &str) {
//     println!("Time elapsed for {}: {}s", name, elapsed.as_secs_f64());
//     println!("Hex string: {}", hex_string);
// }

// fn general_main(){
//     const N: usize = 4;
//     let mut game = game_of_life::Slice::new(N);
//     game.randomize();

//     let mut game_naive = game.clone();
//     let mut game_optimized = game.clone();

//     // println!("Initial Game: {}", game.to_hex_string());
//     // println!("Initial Game Naive: {}", game_naive.to_hex_string());
//     // println!("Initial Game Naive Optimized: {}", game_optimized.to_hex_string());

//     println!(
//         "Naive Start: {}\n{}",
//         game_naive.to_hex_string(),
//         game_naive
//     );
//     println!(
//         "Optimized Start: {}\n{}",
//         game_optimized.to_hex_string(),
//         game_optimized
//     );

//     for i in 0..10 {
//         println!("Frame: {} ", i);
//         game_naive.next_generation_naive();
//         println!("Naive: {}\n{}", game_naive.to_hex_string(), game_naive);
//         game_optimized.next_generation_naive_optimized();
//         println!(
//             "Optimized: {}\n{}",
//             game_optimized.to_hex_string(),
//             game_optimized
//         );
//     }

//     // let start_naive = std::time::Instant::now();
//     // for _ in 0..10000 {
//     //     game_naive.next_generation_naive();
//     //     // println!("Naive: {}", zgame_naive);
//     // }
//     // let elapsed_naive = start_naive.elapsed();

//     // let start_optimized = std::time::Instant::now();
//     // for _ in 0..10000 {
//     //     game_optimized.next_generation_naive_optimized();
//     //     // println!("Optimized: {}", game_optimized);
//     // }
//     // let elapsed_optimized = start_optimized.elapsed();

//     // print_result("Naive", elapsed_naive, &game_naive.to_hex_string());
//     // print_result("Naive Optimized", elapsed_optimized, &game_optimized.to_hex_string());
// }
