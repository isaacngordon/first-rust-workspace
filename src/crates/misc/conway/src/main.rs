use conway::game_of_life;
mod ui;
use bevy::{prelude::*, window::PresentMode};

struct GameOfLife {
    slice: game_of_life::Slice,
}

const DEFAULT_WINDOW_WIDTH: f32 = 500.0;
const DEFAULT_WINDOW_HEIGHT: f32 = 500.0;
const DEFAULT_SLICE_SIZE: usize = 10;
const DEFAULT_CELL_SIZE: f32 = 10.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Conway's Game of Life".into(),
                resolution: (DEFAULT_WINDOW_WIDTH, DEFAULT_WINDOW_HEIGHT).into(),
                // Add more window settings here as needed...
                ..default()
            }),
            ..default()
        }))
        .add_plugins(ui::MainMenuPlugin)
        // .insert_resource(GameOfLife {
        //     slice: game_of_life::Slice::new(10), // Set the size of your grid
        // })
        // .add_startup_system(setup.system())
        // .add_system(update_game_of_life.system())
        // .add_system(render_game_of_life.system())
        .run();
}

// fn setup(mut commands: Commands) {
//     commands.spawn_bundle(OrthographicCameraBundle::new_2d());
// }

// impl Resource for GameOfLife {}

// fn update_game_of_life(time: Res<Time>, mut game_of_life: ResMut<GameOfLife>) {
//     // Update the game state every second or so
//     if time.seconds_since_startup() % 1.0 < 0.05 {
//         game_of_life.slice.next_generation_naive(); // Or use your optimized version
//     }
// }

// struct Cell;

// fn render_game_of_life(
//     mut commands: Commands,
//     game_of_life: Res<GameOfLife>,
//     mut materials: ResMut<Assets<ColorMaterial>>,
//     mut query: Query<(Entity, &Cell)>,
// ) {
//     // First, let's clear the existing cells
//     for (entity, _) in query.iter() {
//         commands.entity(entity).despawn();
//     }

//     // Now, let's draw the new state
//     for (i, &alive) in game_of_life.slice.cells.iter().enumerate() {
//         let x = (i % game_of_life.slice.n) as f32;
//         let y = (i / game_of_life.slice.n) as f32;

//         let color = if alive {
//             Color::rgb(0.0, 1.0, 0.0) // Green for alive
//         } else {
//             Color::rgb(0.1, 0.1, 0.1) // Dark for dead
//         };

//         commands.spawn_bundle(SpriteBundle {
//             material: materials.add(color.into()),
//             transform: Transform::from_xyz(x * 10.0, y * 10.0, 0.0), // Adjust the multiplier for cell size
//             sprite: Sprite::new(Vec2::new(10.0, 10.0)), // Cell size
//             ..Default::default()
//         }).insert(Cell);
//     }
// }




// IGNORE BELOW THIS LINE
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
