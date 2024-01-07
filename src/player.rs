use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::character::{self, Character, ProjectileShooter};
use crate::combat::{ProjectileStats, SpawnProjectileEvent};
use crate::graphics::GameAssets;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, player_input);
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
        .insert(ProjectileShooter {
            projectile_stats: ProjectileStats { speed: 25000.0 },
        })
        .insert(Player);
}

fn player_input(
    mut player_query: Query<(&ProjectileShooter, &Transform, &mut Character), With<Player>>,
    mut shoot_event_writer: EventWriter<SpawnProjectileEvent>,
    key_input: Res<Input<KeyCode>>,
    mouse_input: Res<Input<MouseButton>>,
    windows: Query<&Window>,
    camera: Query<(&Camera, &GlobalTransform)>,
) {
    let (shooter, transform, mut character) = player_query.single_mut();
    let window = windows.single();
    let (camera, camera_transform) = camera.single();

    character.input = Vec2::ZERO;

    if key_input.pressed(KeyCode::W) {
        character.input.y = 1.0;
    }

    if key_input.pressed(KeyCode::S) {
        character.input.y = -1.0;
    }

    if key_input.pressed(KeyCode::A) {
        character.input.x = -1.0;
    }

    if key_input.pressed(KeyCode::D) {
        character.input.x = 1.0;
    }

    if mouse_input.pressed(MouseButton::Left) {
        if let Some(world_position) = window
            .cursor_position()
            .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor))
        {
            let position = transform.translation.truncate();

            let direction = character::direction_to(position, world_position);

            shoot_event_writer.send(SpawnProjectileEvent {
                projectile_stats: shooter.projectile_stats.clone(),
                direction,
                start_position: position,
            });
        }
    }
}
