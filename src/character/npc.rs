use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::prelude::*;

use crate::{
    animation::WobbleBundle,
    combat::{self, Immunity, ProjectileStats, ENEMY_GROUP, PLAYER_GROUP, PROJECTILE_GROUP},
    graphics::GameAssets,
};

use super::{player::Player, Character, ProjectileShooter, ShootEvent};

const CHASE_RANGE: f32 = 250.0;

pub struct NpcPlugin;

impl Plugin for NpcPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, npc_update);
    }
}

#[derive(Component)]
pub struct NpcController {
    kind: NpcKind,
    pub target: Option<NpcTarget>,
    target_change: Timer,
}

#[derive(PartialEq)]
pub enum NpcTarget {
    Position(Vec2),
    Character(Entity),
}

pub enum NpcKind {
    /// Friendly NPCs will only attack while provoked
    Friendly,
    /// Hostile NPCs will start chasing you when you get close
    Hostile,
    /// Very Hostile NPCs will chase you after being spawned
    VeryHostile,
}

pub fn spawn_npc(
    commands: &mut Commands,
    game_assets: &Res<GameAssets>,
    position: Vec2,
    kind: NpcKind,
) {
    let friendly = commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(120.0, 120.0)),
                ..Default::default()
            },
            transform: Transform::from_translation(position.extend(0.0)),
            texture: game_assets.humanoid.clone(),
            ..Default::default()
        })
        .insert(RigidBody::Dynamic)
        .insert(Collider::cuboid(40.0, 60.0))
        .insert(Velocity::zero())
        .insert(LockedAxes::ROTATION_LOCKED)
        .insert(CollisionGroups::new(
            Group::from_bits_truncate(ENEMY_GROUP),
            Group::from_bits_truncate(PROJECTILE_GROUP | 0b0001),
        ))
        .insert(Character {
            max_health: 25.0,
            health: 25.0,

            input: Vec2::ZERO,
            last_x: -1.0,
            speed: 7250.0,
            accel: 3.9,
            damp: 5.0,
        })
        .insert(WobbleBundle::new(Vec3::ONE))
        .insert(Immunity(Timer::from_seconds(0.25, TimerMode::Once)))
        .insert(ProjectileShooter {
            attack_speed: Timer::from_seconds(0.5, TimerMode::Once),
            projectile_stats: ProjectileStats {
                damage: 4.5,
                knockback: 8000.0,
                speed: 25000.0,
                life_time: Timer::from_seconds(0.20, TimerMode::Once),
            },
        })
        .insert(NpcController {
            target: None,
            target_change: Timer::from_seconds(
                rand::thread_rng().gen_range(5.0..15.0),
                TimerMode::Repeating,
            ),
            kind,
        })
        .id();

    combat::healthbar::spawn_healthbar(commands, Vec2::new(0.0, -80.0), friendly);
}

fn npc_update(
    mut npc_query: Query<(
        Entity,
        &Transform,
        &mut NpcController,
        &mut Character,
        &mut Velocity,
        &ProjectileShooter,
    )>,
    player_query: Query<(Entity, &Transform), With<Player>>,
    transform_query: Query<&Transform>,
    mut shoot_event_writer: EventWriter<ShootEvent>,
    time: Res<Time>,
) {
    let mut rng = rand::thread_rng();

    for (npc_entity, transform, mut npc, mut character, mut velocity, projectile_shooter) in
        npc_query.iter_mut()
    {
        npc.target_change.tick(time.delta());

        // NPC Kind-specific behaviour
        match npc.kind {
            NpcKind::Hostile => {
                if let Ok((player_entity, player_transform)) = player_query.get_single() {
                    let player_pos = player_transform.translation.truncate();

                    if transform.translation.truncate().distance(player_pos) < CHASE_RANGE {
                        npc.target = Some(NpcTarget::Character(player_entity));
                    }
                }
            }
            NpcKind::VeryHostile => {
                if let Ok((player_entity, _)) = player_query.get_single() {
                    npc.target = Some(NpcTarget::Character(player_entity));
                }
            }
            NpcKind::Friendly => {}
        }

        // Targets
        if let Some(target) = &npc.target {
            match target {
                NpcTarget::Position(target_pos) => {
                    let position = transform.translation.truncate();

                    if (position.distance(*target_pos)) > 7.5 {
                        character.input =
                            super::direction_to(transform.translation.truncate(), *target_pos);
                    } else {
                        npc.target = None;
                    }
                }
                NpcTarget::Character(entity) => {
                    if let Ok(target_transform) = transform_query.get(*entity) {
                        let target_pos = target_transform.translation.truncate();

                        let position = transform.translation.truncate();

                        character.input = super::direction_to(position, target_pos);

                        // Used to predict whether the projectile could have a chance to hit the target
                        let possible_range = (projectile_shooter.projectile_stats.speed
                            * projectile_shooter
                                .projectile_stats
                                .life_time
                                .duration()
                                .as_secs_f32())
                            / 25.0;

                        if position.distance(target_pos) < possible_range {
                            shoot_event_writer.send(ShootEvent {
                                entity: npc_entity,
                                target: target_pos,
                                target_group: PLAYER_GROUP,
                            });
                        }

                        if position.distance(target_pos) > CHASE_RANGE {
                            npc.target = None;
                        }
                    }
                }
            }
        } else {
            character.input = Vec2::ZERO;
            velocity.linvel = velocity
                .linvel
                .lerp(Vec2::ZERO, character.damp * time.delta_seconds());
        }

        if npc.target_change.just_finished() {
            if let Some(target) = &npc.target {
                if let NpcTarget::Position(_) = target {
                    npc.target = None;
                }
            } else {
                let position = transform.translation.truncate();
                let x = rng.gen_range(position.x - 250.0..position.x + 250.0) as f32;
                let y = rng.gen_range(position.y - 250.0..position.y + 250.0) as f32;
                npc.target = Some(NpcTarget::Position(Vec2::new(x, y)));
            }
        }
    }
}
