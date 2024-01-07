use bevy::{math::dvec2, prelude::*};
use bevy_rapier2d::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(FixedUpdate, player_movement);
    }
}

#[derive(Component)]
pub struct Player;

pub fn spawn_player(mut commands: Commands) {
    commands
        .spawn(RigidBody::Dynamic)
        .insert(Collider::cuboid(50.0, 50.0))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, 100.0, 0.0)))
        .insert(Velocity::zero())
        .insert(LockedAxes::ROTATION_LOCKED)
        .insert(Player);
}

pub fn player_movement(
    mut player_query: Query<&mut Velocity, With<Player>>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let delta = time.delta_seconds();
    let mut velocity = player_query.single_mut();

    let mut input_vector = Vec2::ZERO;

    velocity.linvel = Vec2::ZERO;
    velocity.angvel = 0.0;

    if input.pressed(KeyCode::W) {
        input_vector.y = 1.0;
    }

    if input.pressed(KeyCode::S) {
        input_vector.y = -1.0;
    }

    if input.pressed(KeyCode::A) {
        input_vector.x = -1.0;
    }

    if input.pressed(KeyCode::D) {
        input_vector.x = 1.0;
    }

    input_vector = input_vector.normalize_or_zero();
    input_vector *= 5000.0 * delta;

    velocity.linvel = input_vector;
}
