use core::panic;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{character::Character, graphics::GameAssets};

pub struct CombatPlugin;

pub const PROJECTILE_GROUP: u32 = 0b1000;
pub const PLAYER_GROUP: u32 = 0b0010;
pub const ENEMY_GROUP: u32 = 0b0100;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnProjectileEvent>()
            .add_event::<CharacterAttackEvent>()
            .add_systems(
                Update,
                (
                    projectile_spawn_event,
                    character_attack_event,
                    collision_event,
                ),
            )
            .add_systems(FixedUpdate, handle_projectiles);
    }
}

#[derive(Event)]
pub struct SpawnProjectileEvent {
    pub projectile_stats: ProjectileStats,
    pub direction: Vec2,
    pub start_position: Vec2,
    pub target_group: u32,
}

#[derive(Event)]
pub struct CharacterAttackEvent {
    pub victim: Entity,
    pub projectile: Projectile,
}

#[derive(Clone)]
pub struct ProjectileStats {
    pub damage: f32,
    pub knockback: f32,
    pub speed: f32,
    pub life_time: Timer,
}

#[derive(Component, Clone)]
pub struct Projectile {
    pub stats: ProjectileStats,
    pub direction: Vec2,
}

fn spawn_projectile(
    commands: &mut Commands,
    game_assets: &Res<GameAssets>,
    position: Vec2,
    direction: Vec2,
    projectile_stats: ProjectileStats,
    target_group: u32,
) {
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(64.0, 48.0)),
                ..Default::default()
            },
            transform: Transform {
                translation: position.extend(1.0),
                rotation: Quat::from_rotation_arc_2d(Vec2::Y, direction),
                ..Default::default()
            },
            texture: game_assets.slash.clone(),
            ..Default::default()
        })
        .insert(RigidBody::Dynamic)
        .insert(Collider::cuboid(32.0, 24.0))
        .insert(Sensor)
        .insert(CollisionGroups::new(
            Group::from_bits_truncate(PROJECTILE_GROUP),
            Group::from_bits_truncate(target_group | 0b0001),
        ))
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(LockedAxes::ROTATION_LOCKED)
        .insert(Velocity::zero())
        .insert(Projectile {
            stats: projectile_stats,
            direction,
        });
}

fn handle_projectiles(
    mut commands: Commands,
    mut projectile_query: Query<(Entity, &mut Projectile, &mut Transform, &mut Velocity)>,
    time: Res<Time>,
) {
    let delta = time.delta_seconds();

    for (entity, mut projectile, mut transform, mut velocity) in projectile_query.iter_mut() {
        // Maybe it's not a good idea to make the game crash over this
        if projectile.stats.life_time.mode() != TimerMode::Once {
            panic!(
                "Attempted to handle the behaviour of a projectile that has a repeating lifetime"
            );
        }

        projectile.stats.life_time.tick(time.delta());

        if projectile.stats.life_time.finished() {
            commands.entity(entity).despawn_recursive();
        }

        transform.rotation = Quat::from_rotation_arc_2d(Vec2::Y, projectile.direction);

        velocity.linvel = projectile.direction * projectile.stats.speed * delta;
    }
}

fn collision_event(
    mut commands: Commands,
    projectile_query: Query<&Projectile>,
    character_query: Query<&Character>,
    mut collision_events: EventReader<CollisionEvent>,
    mut attack_event_writer: EventWriter<CharacterAttackEvent>,
) {
    for event in collision_events.read() {
        if let CollisionEvent::Started(first, second, _) = event {
            // We check each side two times because both entities may not be always in the same order
            // TODO: Follow the DRY principle
            if let Ok(hit_projectile) = projectile_query.get(*first) {
                if character_query.get(*second).is_ok() {
                    attack_event_writer.send(CharacterAttackEvent {
                        victim: *second,
                        projectile: (*hit_projectile).clone(),
                    });
                }
                commands.entity(*first).despawn_recursive();
                continue; // If this branch happened, there's no reason to check a second time, so we carry on
            }

            if let Ok(hit_projectile) = projectile_query.get(*second) {
                if character_query.get(*first).is_ok() {
                    attack_event_writer.send(CharacterAttackEvent {
                        victim: *first,
                        projectile: (*hit_projectile).clone(),
                    });
                }
                commands.entity(*second).despawn_recursive();
            }
        }
    }
}

fn character_attack_event(
    mut attack_events: EventReader<CharacterAttackEvent>,
    mut character_query: Query<(&mut Character, &mut Velocity)>,
    time: Res<Time>,
) {
    let delta = time.delta_seconds();

    for event in attack_events.read() {
        if let Ok((mut character, mut velocity)) = character_query.get_mut(event.victim) {
            character.health -= event.projectile.stats.damage;

            velocity.linvel =
                event.projectile.direction * (event.projectile.stats.knockback * delta);
        }
    }
}

fn projectile_spawn_event(
    mut commands: Commands,
    game_assets: Res<GameAssets>,
    mut event_reader: EventReader<SpawnProjectileEvent>,
) {
    for event in event_reader.read() {
        spawn_projectile(
            &mut commands,
            &game_assets,
            event.start_position,
            event.direction,
            event.projectile_stats.clone(),
            event.target_group,
        );
    }
}
