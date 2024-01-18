use bevy::prelude::*;

use crate::game_of_life::GlobalDefaults;
use crate::game_of_life::Slice;
use crate::ui::NextStepEvent;

const SPRITE_SIZE: f32 = 252.0;

pub struct GameOfLifePlugin;

impl Plugin for GameOfLifePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ContinuousSteps {
            toggle: true,
            last_draw_time: 0.0,
        })
            .add_systems(Startup, setup)
            .add_systems(Update, update_game_of_life);
    }
}

#[derive(Component)]
struct Cell {
    alive: CellState,
}

enum CellState {
    Alive,
    Dead,
}

#[derive(Resource)]
struct CellTexture {
    alive: Handle<Image>,
    dead: Handle<Image>,
}

#[derive(Resource)]
struct ContinuousSteps {
    toggle: bool,
    last_draw_time: f64,
}

#[derive(Resource)]
pub struct GameOfLife {
    slice: Slice,
}

fn setup(
    mut commands: Commands,
    global_defaults: Res<GlobalDefaults>,
    asset_server: Res<AssetServer>,
) {
    let mut game_of_life = GameOfLife {
        slice: Slice::new(global_defaults.game_size),
    };
    let cell_textures = CellTexture {
        alive: asset_server.load("sprites/alive_cell.png"),
        dead: asset_server.load("sprites/dead_cell.png"),
    };

    game_of_life.slice.randomize();
    spawn_game_of_life_cells(&mut commands, &cell_textures, &game_of_life);

    commands.insert_resource(game_of_life);
    commands.insert_resource(cell_textures);
}

fn spawn_game_of_life_cells(
    commands: &mut Commands,
    cell_textures: &CellTexture,
    game_of_life: &GameOfLife,
) {
    for x in 0..game_of_life.slice.get_size() {
        for y in 0..game_of_life.slice.get_size() {
            let cell = game_of_life.slice.get(x, y);
            commands
                .spawn(SpriteBundle {
                    sprite: Sprite {
                        ..Default::default()
                    },
                    transform: Transform {
                        translation: Vec3::new(
                            (x as f32) * SPRITE_SIZE,
                            (y as f32) * SPRITE_SIZE,
                            0.0,
                        ),
                        scale: Vec3::ONE,
                        ..Default::default()
                    },
                    texture: if cell {
                        cell_textures.alive.clone()
                    } else {
                        cell_textures.dead.clone()
                    },
                    ..Default::default()
                })
                .insert(Cell {
                    alive: if cell {
                        CellState::Alive
                    } else {
                        CellState::Dead
                    },
                });
        }
    }
}

fn update_game_of_life(
    mut commands: Commands,
    time: Res<Time>,
    mut continuous: ResMut<ContinuousSteps>,
    global_defaults: Res<GlobalDefaults>,
    cell_textures: Res<CellTexture>,
    mut next_step_events: EventReader<NextStepEvent>,
    mut game_of_life: ResMut<GameOfLife>,
    spawned_cells: Query<(Entity, &mut Cell)>,
) {
    // Update the game state at the default FPS rate if continuous steps are enabled
    // Otherwise, only update the game state when the user clicks the "Next" button ie NextStepEvent

    match (continuous.toggle, next_step_events.read().next().is_some()) {
        // not continuous and next step event is true, so continue
        (false, true) => {}
        // no reason to update, lets return
        (false, false) => {
            return;
        }
        // continuous is true, so we need to check if it is time to update
        (true, _) => {
            if time.elapsed_seconds_f64() - continuous.last_draw_time < (1.0 / global_defaults.continuous_frame_rate).into() {
                return;
            }
        }
    }

    // clear the board
    for (entity, _) in spawned_cells.iter() {
        commands.entity(entity).despawn();
    }

    // next step
    game_of_life.slice.next_generation_naive_optimized();

    // spawn the new board
    spawn_game_of_life_cells(&mut commands, &cell_textures, &game_of_life);
    continuous.last_draw_time = time.elapsed_seconds_f64();
}

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
