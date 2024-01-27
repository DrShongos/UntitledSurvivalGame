use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use state::GameState;

mod animation;
mod asset;
mod character;
mod combat;
mod debug;
mod state;
mod world;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugins(bevy_tweening::TweeningPlugin)
        .add_plugins((
            debug::DebugPlugin,
            world::WorldPlugin,
            character::CharacterPlugin,
            asset::AssetPlugin,
            combat::CombatPlugin,
            animation::AnimationPlugin,
        ))
        .add_state::<GameState>()
        .add_systems(Startup, setup_camera)
        .add_systems(OnEnter(GameState::InGame), setup_physics)
        .run();
}

fn setup_camera(mut commands: Commands) {
    let camera_bundle = Camera2dBundle::default();
    //camera_bundle.projection.scale = 7.0;
    commands.spawn(camera_bundle);
}

fn setup_physics(mut rapier_config: ResMut<RapierConfiguration>) {
    rapier_config.gravity = Vec2::ZERO;
}
