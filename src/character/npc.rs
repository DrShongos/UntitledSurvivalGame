use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::prelude::*;
use serde::Deserialize;

use crate::{combat::PLAYER_GROUP, state::GameState};

use super::{player::Player, Character, ProjectileShooter, ShootEvent};

const CHASE_RANGE: f32 = 250.0;

pub struct NpcPlugin;

impl Plugin for NpcPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, npc_update.run_if(in_state(GameState::InGame)));
    }
}

#[derive(Component)]
pub struct NpcController {
    pub kind: NpcKind,
    pub target: Option<NpcTarget>,
    pub target_change: Timer,
}

#[derive(PartialEq)]
pub enum NpcTarget {
    Position(Vec2),
    Character(Entity),
}

#[derive(Clone, Debug, Deserialize)]
pub enum NpcKind {
    /// Friendly NPCs will only attack while provoked
    Friendly,
    /// Hostile NPCs will start chasing you when you get close
    Hostile,
    /// Very Hostile NPCs will chase you after being spawned
    VeryHostile,
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
