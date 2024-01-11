use bevy::prelude::*;

pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, animated_update);
    }
}

pub enum AnimationKind {
    ProjectileGrow { target_scale: f32 },
    ProjectileVanish,
}

pub struct Animation {
    pub kind: AnimationKind,
}

impl Animation {
    pub fn projectile_grow(target_scale: f32) -> Self {
        Self {
            kind: AnimationKind::ProjectileGrow { target_scale },
        }
    }

    pub fn projectile_vanish() -> Self {
        Self {
            kind: AnimationKind::ProjectileVanish,
        }
    }
}

#[derive(Component)]
pub struct Animated {
    pub current: Option<Animation>,
}

fn animated_update(
    mut commands: Commands,
    mut animated_query: Query<(Entity, &mut Transform, &mut Animated)>,
    time: Res<Time>,
) {
    for (entity, mut transform, mut animated) in animated_query.iter_mut() {
        if let Some(animation) = animated.current.as_mut() {
            match animation.kind {
                AnimationKind::ProjectileGrow { target_scale } => {
                    transform.scale = transform.scale.lerp(
                        Vec2::splat(target_scale).extend(1.0),
                        25.0 * time.delta_seconds(),
                    );
                }
                AnimationKind::ProjectileVanish => {
                    transform.scale = transform
                        .scale
                        .lerp(Vec3::ZERO, 35.0 * time.delta_seconds());

                    if transform.scale.length() < 0.1 {
                        commands.entity(entity).despawn_recursive();
                    }
                }
            }
        }
    }
}
