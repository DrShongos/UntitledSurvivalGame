use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct CharacterPlugin;

impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, move_characters);
    }
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
