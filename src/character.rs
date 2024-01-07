use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::combat::ProjectileStats;

pub struct CharacterPlugin;

impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, move_characters);
    }
}

#[derive(Component)]
pub struct ProjectileShooter {
    pub projectile_stats: ProjectileStats,
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

pub fn direction_to(position: Vec2, target: Vec2) -> Vec2 {
    (target - position).normalize_or_zero()
}
