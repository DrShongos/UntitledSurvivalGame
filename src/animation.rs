use std::time::Duration;

use bevy::prelude::*;
use bevy_tweening::{
    lens::{SpriteColorLens, TransformScaleLens},
    Animator, EaseFunction, RepeatCount, RepeatStrategy, Tween, TweenCompleted,
};

use crate::{character::player::Player, state::GameState};

pub const VANISHING_COMPLETED: u64 = 1;
pub const FLASH_COMPLETED: u64 = 2;

pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<VanishEvent>()
            .add_event::<HitFlashEvent>()
            .add_systems(
                Update,
                (vanish_event, hit_flash_event, process_tween_events),
            );
    }
}

/// A marker component used to check if an entity which was ordered to vanish hasn't been set
/// already. It exists because the vanishing animation gets overriden and entities never get
/// deleted after fully vanishing.
#[derive(Component)]
struct VanishMarker;

#[derive(Event)]
pub struct VanishEvent {
    pub entity: Entity,
}

#[derive(Component)]
struct HitFlashMarker;

#[derive(Event)]
pub struct HitFlashEvent {
    pub entity: Entity,
}

#[derive(Component)]
pub struct Wobble {
    pub start_scale: Vec3,
}

#[derive(Bundle)]
pub struct WobbleBundle {
    wobble: Wobble,
    animator: Animator<Transform>,
}

impl WobbleBundle {
    pub fn new(scale: Vec3) -> Self {
        let mut target_scale = scale;
        target_scale.y -= 0.05;

        WobbleBundle {
            wobble: Wobble { start_scale: scale },
            animator: Animator::new(
                Tween::new(
                    EaseFunction::CubicOut,
                    Duration::from_millis(500),
                    TransformScaleLens {
                        start: scale,
                        end: target_scale,
                    },
                )
                .with_repeat_count(RepeatCount::Infinite)
                .with_repeat_strategy(RepeatStrategy::MirroredRepeat),
            ),
        }
    }
}

fn process_tween_events(
    mut commands: Commands,
    mut reader: EventReader<TweenCompleted>,
    player_query: Query<&Player>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    for event in reader.read() {
        if event.user_data == VANISHING_COMPLETED {
            if player_query.contains(event.entity) {
                game_state.set(GameState::Dead);
            }
            commands.entity(event.entity).despawn_recursive();
        }

        if event.user_data == FLASH_COMPLETED {
            if let Some(mut entity) = commands.get_entity(event.entity) {
                entity.remove::<HitFlashMarker>();
            }
        }
    }
}

fn vanish_event(
    mut commands: Commands,
    transform_query: Query<&Transform>,
    marked: Query<&VanishMarker>,
    mut vanish_events: EventReader<VanishEvent>,
) {
    for event in vanish_events.read() {
        if !marked.contains(event.entity) {
            if let Some(mut entity) = commands.get_entity(event.entity) {
                entity.insert(VanishMarker);
            }

            if let Ok(transform) = transform_query.get(event.entity) {
                let mut tween = Tween::new(
                    EaseFunction::CubicOut,
                    Duration::from_millis(100),
                    TransformScaleLens {
                        start: transform.scale,
                        end: Vec3::ZERO,
                    },
                );
                tween.set_completed_event(VANISHING_COMPLETED);

                if let Some(mut entity) = commands.get_entity(event.entity) {
                    entity.insert(Animator::new(tween));
                }
            }
        }
    }
}

fn hit_flash_event(
    mut commands: Commands,
    mut hit_flash_events: EventReader<HitFlashEvent>,
    marked: Query<&HitFlashMarker>,
    sprite_query: Query<&Sprite>,
) {
    for event in hit_flash_events.read() {
        if let Ok(sprite) = sprite_query.get(event.entity) {
            if !marked.contains(event.entity) {
                if let Some(mut entity) = commands.get_entity(event.entity) {
                    entity.insert(HitFlashMarker);
                }

                let flash = Tween::new(
                    EaseFunction::SineInOut,
                    Duration::from_millis(75),
                    SpriteColorLens {
                        start: sprite.color,
                        end: Color::RED,
                    },
                )
                .with_repeat_count(2)
                .with_repeat_strategy(RepeatStrategy::MirroredRepeat)
                .with_completed_event(FLASH_COMPLETED);

                if let Some(mut entity) = commands.get_entity(event.entity) {
                    entity.insert(Animator::new(flash));
                }
            }
        }
    }
}
