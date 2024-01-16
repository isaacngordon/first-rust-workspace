use bevy::prelude::*;
use crate::game_of_life::Slice;

pub struct GameOfLifePlugin;

impl Plugin for GameOfLifePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}

struct GameOfLife {
    slice: Slice,
}


fn setup(mut commands: Commands) {
    
}

impl Resource for GameOfLife {}

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