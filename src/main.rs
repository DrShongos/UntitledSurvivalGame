use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

mod character;
mod combat;
mod graphics;
mod player;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins((
            character::CharacterPlugin,
            graphics::GraphicsPlugin,
            player::PlayerPlugin,
            combat::CombatPlugin,
        ))
        .add_systems(Startup, (setup_camera, setup_physics))
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn setup_physics(mut commands: Commands, mut rapier_config: ResMut<RapierConfiguration>) {
    rapier_config.gravity = Vec2::ZERO;

    commands
        .spawn(RigidBody::Fixed)
        .insert(Collider::cuboid(100.0, 50.0))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, -100.0, 0.0)));
}
