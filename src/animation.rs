use std::time::Duration;

use bevy::prelude::*;
use bevy_tweening::{lens::TransformScaleLens, Animator, EaseFunction, Tween, TweenCompleted};

pub const VANISHING_COMPLETED: u64 = 1;

pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<VanishEvent>()
            .add_systems(Update, (vanish_event, process_tween_events));
    }
}

/// A marker component used to check if an entity which was ordered to vanish hasn't been set
/// already. It exists because the vanishing animation gets overriden and entities never get
/// deleted after fully vanishing.
#[derive(Component)]
struct VanishMarker;

#[derive(Event)]
pub struct VanishEvent {
    pub target: Entity,
}

fn process_tween_events(mut commands: Commands, mut reader: EventReader<TweenCompleted>) {
    for event in reader.read() {
        if event.user_data == VANISHING_COMPLETED {
            commands.entity(event.entity).despawn_recursive();
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
        if !marked.contains(event.target) {
            if let Some(mut entity) = commands.get_entity(event.target) {
                entity.insert(VanishMarker);
            }

            if let Ok(transform) = transform_query.get(event.target) {
                let mut tween = Tween::new(
                    EaseFunction::CubicOut,
                    Duration::from_millis(100),
                    TransformScaleLens {
                        start: transform.scale,
                        end: Vec3::ZERO,
                    },
                );
                tween.set_completed_event(VANISHING_COMPLETED);

                if let Some(mut entity) = commands.get_entity(event.target) {
                    entity.insert(Animator::new(tween));
                }
            }
        }
    }
}
