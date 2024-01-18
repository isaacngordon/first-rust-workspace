use bevy::prelude::*;
use crate::game_of_life::Slice;

const SPRITE_SIZE: f32 = 252.0;
const DEFAULT_SLICE_SIZE: usize = 4;

pub struct GameOfLifePlugin;

impl Plugin for GameOfLifePlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(GameOfLife::default())
            .add_systems(Startup, setup);
    }
}

#[derive(Component)]
struct Cell {
    alive: CellState,
}

enum CellState {
    Alive,
    Dead
}

#[derive(Resource)]
struct CellTexture {
    alive: Handle<Image>,
    dead: Handle<Image>
}

#[derive(Default)]
struct Continuous(bool);

#[derive(Resource)]
pub struct GameOfLife {
    slice: Slice,
}

impl Default for GameOfLife {
    fn default() -> Self {
        Self {
            slice: Slice::new(DEFAULT_SLICE_SIZE),
        }
    }
}

fn setup(mut commands: Commands, mut game_of_life: Res<GameOfLife>, asset_server: Res<AssetServer>) {
    for x in 0..game_of_life.slice.get_size() {
        for y in 0..game_of_life.slice.get_size() {
            let cell = game_of_life.slice.get(x, y);
            commands
                .spawn(SpriteBundle {
                    sprite: Sprite {
                        ..Default::default()
                    },
                    transform: Transform {
                        translation: Vec3::new((x as f32) * SPRITE_SIZE, (y as f32) * SPRITE_SIZE, 0.0),
                        scale : Vec3::ONE,
                        ..Default::default()
                    }, 
                    texture: asset_server.load("sprites/dead_cell.png"),
                    ..Default::default()
                })
                .insert(Cell {
                    alive: if cell { CellState::Alive } else { CellState::Dead }
                });
        }
    }
    // commands.spawn(Camera2dBundle::default());
    // commands.spawn(SpriteBundle{
    //     texture: asset_server.load("sprites/dead_cell.png"),
    //     ..default()
    // });
    commands.insert_resource(CellTexture {
        alive: asset_server.load("sprites/alive_cell.png"),
        dead: asset_server.load("sprites/dead_cell.png")
    });
}


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