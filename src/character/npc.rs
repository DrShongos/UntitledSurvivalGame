use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::prelude::*;

use crate::{
    combat::{ENEMY_GROUP, PROJECTILE_GROUP},
    graphics::GameAssets,
};

use super::Character;

pub struct NpcPlugin;

impl Plugin for NpcPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, populate_world)
            .add_systems(Update, npc_update);
    }
}

#[derive(Component)]
pub enum NpcKind {
    Friendly {
        target_point: Option<Vec2>,
        target_change: Timer,
    },
    Hostile, // TODO
}

fn populate_world(mut commands: Commands, game_assets: Res<GameAssets>) {
    let mut rng = rand::thread_rng();

    for _ in 0..40 {
        let pos_x = rng.gen_range(-1000.0..1000.0) as f32;
        let pos_y = rng.gen_range(-1000.0..1000.0) as f32;

        spawn_friendly(&mut commands, &game_assets, Vec2::new(pos_x, pos_y));
    }
}

fn spawn_friendly(commands: &mut Commands, game_assets: &Res<GameAssets>, position: Vec2) {
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(70.0, 150.0)),
                ..Default::default()
            },
            transform: Transform::from_translation(position.extend(0.0)),
            texture: game_assets.humanoid.clone(),
            ..Default::default()
        })
        .insert(RigidBody::Dynamic)
        .insert(Collider::cuboid(35.0, 75.0))
        .insert(Velocity::zero())
        .insert(LockedAxes::ROTATION_LOCKED)
        .insert(CollisionGroups::new(
            Group::from_bits_truncate(ENEMY_GROUP),
            Group::from_bits_truncate(PROJECTILE_GROUP | 0b0001),
        ))
        .insert(Character {
            health: 25.0,

            input: Vec2::ZERO,
            speed: 7500.0,
            accel: 3.9,
            damp: 5.0,
        })
        .insert(NpcKind::Friendly {
            target_point: None,
            target_change: Timer::from_seconds(
                rand::thread_rng().gen_range(5.0..15.0),
                TimerMode::Repeating,
            ),
        });
}

fn npc_update(
    mut npc_query: Query<(&Transform, &mut NpcKind, &mut Character, &mut Velocity)>,
    time: Res<Time>,
) {
    let mut rng = rand::thread_rng();

    for (transform, mut npc_kind, mut character, mut velocity) in npc_query.iter_mut() {
        match npc_kind.as_mut() {
            NpcKind::Friendly {
                target_point,
                target_change,
            } => {
                target_change.tick(time.delta());

                if let Some(target) = target_point {
                    let position = transform.translation.truncate();

                    if (position.distance(*target)) > 7.5 {
                        character.input =
                            super::direction_to(transform.translation.truncate(), *target);
                    } else {
                        character.input = Vec2::ZERO;
                        velocity.linvel = velocity
                            .linvel
                            .lerp(Vec2::ZERO, character.damp * time.delta_seconds());
                    }
                } else {
                    character.input = Vec2::ZERO;
                }

                if target_change.just_finished() {
                    if target_point.is_some() {
                        *target_point = None;
                    } else {
                        let position = transform.translation.truncate();
                        let x = rng.gen_range(position.x - 250.0..position.x + 250.0) as f32;
                        let y = rng.gen_range(position.y - 250.0..position.y + 250.0) as f32;
                        *target_point = Some(Vec2::new(x, y));
                    }
                }
            }
            NpcKind::Hostile => {}
        }
    }
}
