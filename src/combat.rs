use core::panic;
use std::time::Duration;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_tweening::{lens::TransformScaleLens, Animator, EaseFunction, Tween};

use crate::{animation::VanishEvent, character::Character, graphics::GameAssets};

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
                    immunity_update,
                ),
            )
            .add_systems(FixedUpdate, handle_projectiles)
            .register_type::<Projectile>()
            .register_type::<Immunity>()
            .register_type::<ProjectileStats>();
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

#[derive(Reflect, Clone)]
pub struct ProjectileStats {
    pub damage: f32,
    pub knockback: f32,
    pub speed: f32,
    pub life_time: Timer,
}

#[derive(Reflect, Component, Clone)]
pub struct Projectile {
    pub stats: ProjectileStats,
    pub direction: Vec2,
}

#[derive(Reflect, Component)]
pub struct Immunity(pub Timer);

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
                scale: Vec3::ZERO,
            },
            texture: game_assets.slash.clone(),
            ..Default::default()
        })
        .insert(RigidBody::Dynamic)
        .insert(Animator::new(Tween::new(
            EaseFunction::CubicOut,
            Duration::from_millis(100),
            TransformScaleLens {
                start: Vec3::ZERO,
                end: Vec3::new(1.0, 1.0, 1.0),
            },
        )))
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
    mut projectile_query: Query<(Entity, &mut Projectile, &mut Transform, &mut Velocity)>,
    mut vanish_writer: EventWriter<VanishEvent>,
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

        if projectile.stats.life_time.just_finished() {
            vanish_writer.send(VanishEvent { target: entity });
        }

        transform.rotation = Quat::from_rotation_arc_2d(Vec2::Y, projectile.direction);

        velocity.linvel = projectile.direction * projectile.stats.speed * delta;
    }
}

fn collision_event(
    projectile_query: Query<&Projectile>,
    character_query: Query<&Character>,
    mut collision_events: EventReader<CollisionEvent>,
    mut attack_event_writer: EventWriter<CharacterAttackEvent>,
    mut projectile_vanish_writer: EventWriter<VanishEvent>,
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
                projectile_vanish_writer.send(VanishEvent { target: *first });
                continue; // If this branch happened, there's no reason to check a second time, so we carry on
            }

            if let Ok(hit_projectile) = projectile_query.get(*second) {
                if character_query.get(*first).is_ok() {
                    attack_event_writer.send(CharacterAttackEvent {
                        victim: *first,
                        projectile: (*hit_projectile).clone(),
                    });
                }
                projectile_vanish_writer.send(VanishEvent { target: *second });
            }
        }
    }
}

fn character_attack_event(
    mut attack_events: EventReader<CharacterAttackEvent>,
    mut character_query: Query<(&mut Character, &mut Immunity, &mut Velocity)>,
    time: Res<Time>,
) {
    let delta = time.delta_seconds();

    for event in attack_events.read() {
        if let Ok((mut character, mut immunity, mut velocity)) =
            character_query.get_mut(event.victim)
        {
            if immunity.0.finished() {
                character.health -= event.projectile.stats.damage;

                velocity.linvel =
                    event.projectile.direction * (event.projectile.stats.knockback * delta);

                immunity.0.reset();
                immunity.0.unpause();
            }
        }
    }
}

fn immunity_update(mut immunity_query: Query<&mut Immunity>, time: Res<Time>) {
    for mut immunity in immunity_query.iter_mut() {
        immunity.0.tick(time.delta());
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
