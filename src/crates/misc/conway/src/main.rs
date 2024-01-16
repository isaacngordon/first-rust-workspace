use conway::game_of_life;
mod ui;
use bevy::{prelude::*, transform};

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
        .add_plugins(game_of_life::GameOfLifePlugin)
        .add_plugins(ui::MainMenuPlugin)
        .add_plugins(CameraPlugin)
        .run();
}

// Camera 
const CAMERA_MOVE_SPEED: f32 = 15.0;
const CAMERA_ZOOM_SPEED: f32 = 1.0;

#[derive(Component)]
struct MainCamera;

#[derive(Component)]
struct CameraMovement {
    plane_speed: Vec3,
    zoom_speed: f32,
}

struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, camera_setup)
            .add_systems(Update, (camera_movement_system, camera_zoom_system));
    }
}

fn camera_setup(mut commands: Commands) {
    commands
        .spawn(Camera2dBundle::default())
        .insert(MainCamera)
        .insert(CameraMovement {
            plane_speed: Vec3::ZERO,
            zoom_speed: 0.0,
        }); 
}

fn camera_movement_system(
    mut camera: Query<(&mut Transform, &mut CameraMovement), With<MainCamera>>,
    keyboard_input: Res<Input<KeyCode>>,
){
    
    let mut move_direction = Vec3::ZERO;
    if keyboard_input.pressed(KeyCode::W) {
        move_direction.y = CAMERA_MOVE_SPEED;
    }
    if keyboard_input.pressed(KeyCode::A) {
        move_direction.x = -CAMERA_MOVE_SPEED;
    }
    if keyboard_input.pressed(KeyCode::S) {
        move_direction.y = -CAMERA_MOVE_SPEED;
    }
    if keyboard_input.pressed(KeyCode::D) {
        move_direction.x = CAMERA_MOVE_SPEED;
    }

    let move_direction = move_direction.normalize_or_zero();
    let (mut transform, mut movement) = camera.iter_mut()
        .next()
        .expect("No transform found on camera MainCamera");

    movement.plane_speed = (move_direction);

    transform.translation += movement.plane_speed;

    if keyboard_input.just_pressed(KeyCode::Space) {
        movement.plane_speed = Vec3::ZERO;
        transform.translation = Vec3::ZERO;
    }
}

// untested
fn camera_zoom_system(
    mut camera: Query<(&mut Transform, &mut CameraMovement), With<MainCamera>>,
    keyboard_input: Res<Input<KeyCode>>,
){
    let mut zoom_direction = 0.0;
    if keyboard_input.pressed(KeyCode::Q) {
        zoom_direction = CAMERA_ZOOM_SPEED;
    }
    if keyboard_input.pressed(KeyCode::E) {
        zoom_direction = -CAMERA_ZOOM_SPEED;
    }

    let (mut transform, mut movement) = camera.iter_mut()
        .next()
        .expect("No transform found on camera MainCamera");

    movement.zoom_speed = zoom_direction;

    transform.scale += Vec3::splat(movement.zoom_speed);

    if keyboard_input.just_pressed(KeyCode::Space) {
        movement.zoom_speed = 0.0;
        transform.scale = Vec3::ONE;
    }
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
