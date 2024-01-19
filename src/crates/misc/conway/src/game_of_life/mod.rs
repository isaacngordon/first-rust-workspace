// import public modules
pub mod plugin;
pub mod game;
pub mod slice;

// import private modules
mod camera;

// exposes all public members of public modules
pub use plugin::*;
pub use game::*;

// selectively exposes public members of private modules
pub use camera::CameraPlugin;
pub use camera::MainCamera;
pub use camera::GlobalDefaults;