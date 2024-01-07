use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::character::Character;
use crate::graphics::GameAssets;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(FixedUpdate, player_movement);
    }
}

#[derive(Component)]
pub struct Player;

fn spawn_player(mut commands: Commands, game_assets: Res<GameAssets>) {
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(70.0, 150.0)),
                ..Default::default()
            },
            transform: Transform::from_xyz(0.0, 100.0, 0.0),
            texture: game_assets.humanoid.clone(),
            ..Default::default()
        })
        .insert(RigidBody::Dynamic)
        .insert(Collider::cuboid(35.0, 75.0))
        .insert(Velocity::zero())
        .insert(LockedAxes::ROTATION_LOCKED)
        .insert(Character {
            input: Vec2::ZERO,
            speed: 6000.0,
            accel: 3.9,
            damp: 5.0,
        })
        .insert(Player);
}

fn player_movement(
    mut player_query: Query<&mut Character, With<Player>>,
    input: Res<Input<KeyCode>>,
) {
    let mut character = player_query.single_mut();

    character.input = Vec2::ZERO;

    if input.pressed(KeyCode::W) {
        character.input.y = 1.0;
    }

    if input.pressed(KeyCode::S) {
        character.input.y = -1.0;
    }

    if input.pressed(KeyCode::A) {
        character.input.x = -1.0;
    }

    if input.pressed(KeyCode::D) {
        character.input.x = 1.0;
    }
}
