use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_tweening::Lerp;

use crate::{
    animation::VanishEvent,
    combat::{ProjectileStats, SpawnProjectileEvent},
    state::GameState,
};

pub mod npc;
pub mod player;

pub struct CharacterPlugin;

impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((npc::NpcPlugin, player::PlayerPlugin))
            .add_event::<ShootEvent>()
            .add_systems(
                FixedUpdate,
                move_characters.run_if(in_state(GameState::InGame)),
            )
            .add_systems(
                Update,
                (
                    character_update,
                    shoot_events,
                    shooter_cooldown,
                    health_regen_update,
                )
                    .run_if(in_state(GameState::InGame)),
            )
            .register_type::<HealthRegen>()
            .register_type::<Character>()
            .register_type::<ProjectileShooter>();
    }
}

#[derive(Event)]
pub struct ShootEvent {
    pub entity: Entity,
    pub target: Vec2,
    pub target_group: u32,
}

#[derive(Reflect, Component)]
pub struct ProjectileShooter {
    pub projectile_stats: ProjectileStats,
    pub attack_speed: Timer,
}

#[derive(Reflect, Component)]
pub struct Character {
    // Combat
    pub max_health: f32,
    pub health: f32,

    // Movement
    pub input: Vec2,
    pub last_x: f32,

    pub speed: f32,
    pub accel: f32,
    pub damp: f32,
}

#[derive(Reflect, Component)]
pub struct HealthRegen {
    pub delay: Timer,
    pub speed: f32,
}

fn move_characters(
    mut character_query: Query<(&mut Character, &mut Transform, &mut Velocity)>,
    time: Res<Time>,
) {
    let delta = time.delta_seconds();

    for (mut character, mut transform, mut velocity) in character_query.iter_mut() {
        let mut input = character.input.normalize_or_zero();
        let input_axis = input;
        input *= character.speed * delta;

        if input != Vec2::ZERO {
            if input_axis.x != 0.0 {
                character.last_x = input_axis.x;
            }

            velocity.linvel = velocity.linvel.lerp(input, character.accel * delta);
            transform.rotation.z = transform
                .rotation
                .z
                .lerp(&(-0.1 * character.last_x), &(2.0 * delta));
        } else {
            velocity.linvel = velocity.linvel.lerp(Vec2::ZERO, character.damp * delta);
            transform.rotation.z = transform.rotation.z.lerp(&(0.0), &(2.0 * delta));
        }
    }
}

fn character_update(
    mut character_query: Query<(Entity, &Character)>,
    mut vanish_event_writer: EventWriter<VanishEvent>,
) {
    for (entity, character) in character_query.iter_mut() {
        if character.health <= 0.0 {
            vanish_event_writer.send(VanishEvent { entity });
        }
    }
}

fn health_regen_update(
    mut regen_query: Query<(&mut Character, &mut HealthRegen)>,
    time: Res<Time>,
) {
    for (mut character, mut health_regen) in regen_query.iter_mut() {
        health_regen.delay.tick(time.delta());

        if health_regen.delay.mode() == TimerMode::Repeating {
            panic!("Attempted to handle a HealthRegen component with a repeating delay");
        }

        if health_regen.delay.finished() {
            if character.health < character.max_health {
                character.health += health_regen.speed;
            }
        }
    }
}

fn shoot_events(
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

                let position = position + (extents * direction);

                spawn_event_writer.send(SpawnProjectileEvent {
                    caster: event.entity,
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

fn shooter_cooldown(mut shooters: Query<&mut ProjectileShooter>, time: Res<Time>) {
    for mut shooter in shooters.iter_mut() {
        shooter.attack_speed.tick(time.delta());
    }
}

pub fn direction_to(position: Vec2, target: Vec2) -> Vec2 {
    (target - position).normalize_or_zero()
}
