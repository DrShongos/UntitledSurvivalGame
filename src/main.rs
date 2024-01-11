use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

mod character;
mod combat;
mod debug;
mod graphics;
mod world;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugins((
            debug::DebugPlugin,
            world::WorldPlugin,
            character::CharacterPlugin,
            graphics::GraphicsPlugin,
            combat::CombatPlugin,
        ))
        .add_systems(Startup, (setup_camera, setup_physics))
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn setup_physics(mut rapier_config: ResMut<RapierConfiguration>) {
    rapier_config.gravity = Vec2::ZERO;
}
