use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::combat::{ProjectileStats, SpawnProjectileEvent};

pub mod npc;
pub mod player;

pub struct CharacterPlugin;

impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((npc::NpcPlugin, player::PlayerPlugin))
            .add_event::<ShootEvent>()
            .add_systems(FixedUpdate, move_characters)
            .add_systems(Update, (shoot_events, shooter_cooldown));
    }
}

#[derive(Event)]
pub struct ShootEvent {
    pub entity: Entity,
    pub target: Vec2,
    pub target_group: u32,
}

#[derive(Component)]
pub struct ProjectileShooter {
    pub projectile_stats: ProjectileStats,
    pub attack_speed: Timer,
}

#[derive(Component)]
pub struct Character {
    pub input: Vec2,
    pub speed: f32,
    pub accel: f32,
    pub damp: f32,
}

fn move_characters(mut character_query: Query<(&Character, &mut Velocity)>, time: Res<Time>) {
    let delta = time.delta_seconds();

    for (character, mut velocity) in character_query.iter_mut() {
        let mut input = character.input.normalize_or_zero();
        input *= character.speed * delta;

        if input != Vec2::ZERO {
            velocity.linvel = velocity.linvel.lerp(input, character.accel * delta);
        } else {
            velocity.linvel = velocity.linvel.lerp(Vec2::ZERO, character.damp * delta);
        }
    }
}

pub fn shoot_events(
    mut shooters: Query<(&mut ProjectileShooter, &mut Transform, &mut Collider)>,
    mut events: EventReader<ShootEvent>,
    mut spawn_event_writer: EventWriter<SpawnProjectileEvent>,
) {
    for event in events.read() {
        if let Ok((mut shooter, transform, collider)) = shooters.get_mut(event.entity) {
            if shooter.attack_speed.finished() {
                let position = transform.translation.truncate();
                let extents = collider.as_cuboid().unwrap().half_extents();

                let direction = direction_to(position, event.target);

                let position = position + (extents * direction * 1.33);

                spawn_event_writer.send(SpawnProjectileEvent {
                    projectile_stats: shooter.projectile_stats.clone(),
                    direction,
                    start_position: position,
                    target_group: event.target_group,
                });

                shooter.attack_speed.reset();
                shooter.attack_speed.unpause();
            }
        }
    }
}

pub fn shooter_cooldown(mut shooters: Query<&mut ProjectileShooter>, time: Res<Time>) {
    for mut shooter in shooters.iter_mut() {
        shooter.attack_speed.tick(time.delta());
    }
}

pub fn direction_to(position: Vec2, target: Vec2) -> Vec2 {
    (target - position).normalize_or_zero()
}
