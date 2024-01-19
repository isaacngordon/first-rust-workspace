use bevy::prelude::*;

pub const CAMERA_MOVE_SPEED: f32 = 50.0;
pub const CAMERA_ZOOM_SPEED: f32 = 0.1;

#[derive(Resource)]
pub struct GlobalDefaults{
    /// The default width of the window.
    pub window_width: f32,
    /// The default height of the window.
    pub window_height: f32,
    /// The default size of the game board.
    pub game_size: usize,
    /// The default continuous frame rate.
    pub continuous_frame_rate: f32,
    /// The default size of the game buffer.
    pub game_buffer_size: usize,
}

/// An empty component that marks an entity as the main camera.
#[derive(Component)]
pub struct MainCamera;

/// A component that stores the movement speed of the camera.
#[derive(Component)]
pub struct CameraMovement {
    plane_speed: Vec3,
    zoom_speed: f32,
}

/// A plugin that adds a camera to the scene.
pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, camera_setup)
            .add_systems(Update, (camera_movement_system, camera_zoom_system));
    }
}

fn camera_setup(mut commands: Commands, global_defaults: Res<GlobalDefaults>) {
    commands
        .spawn(Camera2dBundle{
            // custom transform to center the camera on the screen
            transform: Transform{
                translation: Vec3::new(global_defaults.window_width / 2.0, global_defaults.window_height / 2.0, 0.0),
                scale: Vec3::splat(10.0),
                ..default()
            },
            ..default()
        })
        .insert(MainCamera)
        .insert(CameraMovement {
            plane_speed: Vec3::ZERO,
            zoom_speed: 0.0,
        });
}

fn camera_movement_system(
    mut camera: Query<(&mut Transform, &mut CameraMovement), With<MainCamera>>,
    global_defaults: Res<GlobalDefaults>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    let mut move_direction = Vec3::ZERO;

    // Move the camera with WASD
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

    let move_direction = move_direction;
    let (mut transform, mut movement) = camera
        .iter_mut()
        .next()
        .expect("No transform found on camera MainCamera");

    movement.plane_speed = move_direction;

    transform.translation += movement.plane_speed;

    // Reset the camera to the default position when the space bar is pressed
    if keyboard_input.just_pressed(KeyCode::Space) {
        movement.plane_speed = Vec3::ZERO;
        transform.translation = Vec3::new(global_defaults.window_width / 2.0, global_defaults.window_height / 2.0, 0.0)
    }
}

fn camera_zoom_system(
    mut camera: Query<(&mut Transform, &mut CameraMovement), With<MainCamera>>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    let mut zoom_direction = 0.0;

    // Zoom in/out with Q/E
    if keyboard_input.pressed(KeyCode::Q) {
        zoom_direction = CAMERA_ZOOM_SPEED;
    }
    if keyboard_input.pressed(KeyCode::E) {
        zoom_direction = -CAMERA_ZOOM_SPEED;
    }

    let (mut transform, mut movement) = camera
        .iter_mut()
        .next()
        .expect("No transform found on camera MainCamera");

    movement.zoom_speed = zoom_direction;

    transform.scale += Vec3::splat(movement.zoom_speed);

    // Reset the camera to the default zoom level when the space bar is pressed
    if keyboard_input.just_pressed(KeyCode::Space) {
        movement.zoom_speed = 0.0;
        transform.scale = Vec3::splat(10.0);
    }
}
