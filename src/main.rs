mod camera;
mod debug;
mod movement;
mod spaceship;
mod asteroids;
mod light;
mod asset_loader;
mod input;
mod collision_detection;

use bevy::prelude::*;
use camera::CameraPlugin;
// use debug::DebugPlugin;
use movement::MovementPlugin;
use spaceship::SpaceshipPlugin;
use asteroids::AsteroidPlugin;
use light::LightingPlugin;
use asset_loader::AssetLoaderPlugin;
use input::InputPlugin;
use collision_detection::CollisionDetectionPlugin;
fn main() {

    // This app loops forever at 60 fps
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.1, 0.0, 0.15)))
        .insert_resource(AmbientLight {
            color: Color::default(),
            brightness: 0.75
        })
        .add_plugins((
            DefaultPlugins,
            InputPlugin,
            AssetLoaderPlugin,
            MovementPlugin,
            LightingPlugin,
            //DebugPlugin,
            SpaceshipPlugin,
            AsteroidPlugin,
            CameraPlugin,
            CollisionDetectionPlugin,
        ))
        .run();
}

