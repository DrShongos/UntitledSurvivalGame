use crate::animation::WobbleBundle;
use crate::asset::GameSprites;
use crate::character::{Character, ProjectileShooter, ShootEvent};
use crate::combat::{self, Immunity, ProjectileStats, ENEMY_GROUP, PLAYER_GROUP, PROJECTILE_GROUP};
use crate::state::GameState;
use crate::world::{prepare_world, MAX_WORLD_X, MAX_WORLD_Y, MIN_WORLD_X, MIN_WORLD_Y};
use bevy::prelude::*;
use bevy_rapier2d::na::clamp;
use bevy_rapier2d::prelude::*;

use super::HealthRegen;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::PreparingWorld),
            spawn_player.before(prepare_world),
        )
        .add_systems(FixedUpdate, camera_follow)
        .add_systems(Update, player_input);
    }
}

#[derive(Component)]
pub struct Player;

fn spawn_player(
    mut commands: Commands,
    mut game_sprites: ResMut<GameSprites>,
    asset_server: Res<AssetServer>,
) {
    let player = commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(120.0, 120.0)),
                ..Default::default()
            },
            transform: Transform::from_xyz(0.0, 100.0, 0.0),
            texture: game_sprites.get_or_load(&"human-normal.png".to_string(), &asset_server),
            ..Default::default()
        })
        .insert(RigidBody::Dynamic)
        .insert(Collider::cuboid(40.0, 60.0))
        .insert(Velocity::zero())
        .insert(LockedAxes::ROTATION_LOCKED)
        .insert(CollisionGroups::new(
            Group::from_bits_truncate(PLAYER_GROUP),
            Group::from_bits_truncate(PROJECTILE_GROUP | 0b0001),
        ))
        .insert(Character {
            max_health: 30.0,
            health: 30.0,

            input: Vec2::ZERO,
            last_x: -1.0,
            speed: 7500.0,
            accel: 3.9,
            damp: 5.0,
        })
        .insert(ProjectileShooter {
            projectile_stats: ProjectileStats {
                damage: 4.5,
                knockback: 8000.0,
                speed: 25000.0,
                life_time: Timer::from_seconds(0.20, TimerMode::Once),
            },
            attack_speed: Timer::from_seconds(0.5, TimerMode::Once),
        })
        .insert(Immunity(Timer::from_seconds(0.25, TimerMode::Once)))
        .insert(HealthRegen {
            delay: Timer::from_seconds(6.0, TimerMode::Once),
            speed: 0.05,
        })
        .insert(WobbleBundle::new(Vec3::ONE))
        .insert(Name::new("Player"))
        .insert(Player)
        .id();

    combat::healthbar::spawn_healthbar(&mut commands, Vec2::new(0.0, -80.0), player);
}

fn camera_follow(
    player_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<&mut Transform, (With<Camera>, Without<Player>)>,
    time: Res<Time>,
) {
    if let Ok(mut camera_transform) = camera_query.get_single_mut() {
        if let Ok(player_transform) = player_query.get_single() {
            camera_transform.translation = camera_transform
                .translation
                .lerp(player_transform.translation, 1.5 * time.delta_seconds());

            camera_transform.translation.x = clamp(
                camera_transform.translation.x,
                MIN_WORLD_X + 150.0,
                MAX_WORLD_X - 150.0,
            );

            camera_transform.translation.y = clamp(
                camera_transform.translation.y,
                MIN_WORLD_Y - 100.0,
                MAX_WORLD_Y + 100.0,
            );
        }
    }
}

fn player_input(
    mut player_query: Query<(Entity, &mut Character), With<Player>>,
    mut shoot_event_writer: EventWriter<ShootEvent>,
    key_input: Res<Input<KeyCode>>,
    mouse_input: Res<Input<MouseButton>>,
    windows: Query<&Window>,
    camera: Query<(&Camera, &GlobalTransform)>,
) {
    if let Ok((entity, mut character)) = player_query.get_single_mut() {
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
                shoot_event_writer.send(ShootEvent {
                    entity,
                    target: world_position,
                    target_group: ENEMY_GROUP,
                });
            }
        }
    }
}
