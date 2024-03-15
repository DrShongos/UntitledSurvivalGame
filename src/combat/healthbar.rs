use bevy::prelude::*;

use crate::character::Character;

pub struct HealthbarPlugin;

impl Plugin for HealthbarPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_healthbars);
    }
}

/// The Component used for tracking healthbar behaviour.
/// Healthbars are entities because world-space ui is not supported in Bevy yet.
#[derive(Component)]
pub struct HealthBar {
    target: Entity,
    offset_position: Vec2,
}

const HEALTHBAR_WIDTH: f32 = 64.0;
const HEALTHBAR_HEIGHT: f32 = 8.0;

const CONTAINER_OFFSET: f32 = 6.0;

pub fn spawn_healthbar(commands: &mut Commands, offset: Vec2, tracked_entity: Entity) {
    let container = commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::DARK_GRAY,
                custom_size: Some(Vec2::new(
                    HEALTHBAR_WIDTH + CONTAINER_OFFSET,
                    HEALTHBAR_HEIGHT + CONTAINER_OFFSET,
                )),
                ..Default::default()
            },
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, -1.0)),
            ..Default::default()
        })
        .id();

    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::DARK_GREEN,
                custom_size: Some(Vec2::new(
                    HEALTHBAR_WIDTH,
                    HEALTHBAR_HEIGHT + CONTAINER_OFFSET,
                )),
                ..Default::default()
            },
            transform: Transform::from_translation(offset.extend(-2.0)),
            ..Default::default()
        })
        .insert(GlobalTransform::default())
        .insert(HealthBar {
            target: tracked_entity,
            offset_position: offset,
        })
        .push_children(&[container]);
}

fn handle_healthbars(
    mut commands: Commands,
    mut healthbars: Query<(Entity, &HealthBar, &mut Sprite, &mut Transform)>,
    characters: Query<(&Character, &Transform), Without<HealthBar>>,
) {
    for (bar_entity, healthbar, mut sprite, mut healthbar_transform) in healthbars.iter_mut() {
        if let Ok((character, character_transform)) = characters.get(healthbar.target) {
            healthbar_transform.translation =
                character_transform.translation + healthbar.offset_position.extend(0.0);
            // TODO: Figure out why the healthbar isn't fully empty when at 0 health
            let percentage = character.health / character.max_health;
            sprite.custom_size = Some(Vec2::new(HEALTHBAR_WIDTH * percentage, HEALTHBAR_HEIGHT));
        } else {
            // If the components cannot get fetched from the query, this means that the tracked
            // entity is either dead or shouldn't have the healthbar in the first place. Either
            // way, it gets deleted.
            commands.entity(bar_entity).despawn_recursive();
        }
    }
}
