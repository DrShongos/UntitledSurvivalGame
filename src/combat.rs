use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::graphics::GameAssets;

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnProjectileEvent>()
            .add_systems(Update, projectile_spawn_event)
            .add_systems(FixedUpdate, handle_projectiles);
    }
}

#[derive(Event)]
pub struct SpawnProjectileEvent {
    pub projectile_stats: ProjectileStats,
    pub direction: Vec2,
    pub start_position: Vec2,
}

#[derive(Clone, Copy)]
pub struct ProjectileStats {
    pub speed: f32,
}

#[derive(Component)]
pub struct Projectile {
    pub stats: ProjectileStats,
    pub direction: Vec2,
}

fn spawn_projectile(
    commands: &mut Commands,
    game_assets: &Res<GameAssets>,
    position: Vec2,
    direction: Vec2,
    projectile_stats: ProjectileStats,
) {
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(48.0, 48.0)),
                ..Default::default()
            },
            transform: Transform::from_translation(position.extend(1.0)),
            texture: game_assets.slash.clone(),
            ..Default::default()
        })
        .insert(RigidBody::Dynamic)
        .insert(Collider::cuboid(24.0, 24.0))
        .insert(Sensor)
        .insert(LockedAxes::ROTATION_LOCKED)
        .insert(Velocity::zero())
        .insert(Projectile {
            stats: projectile_stats,
            direction,
        });
}

fn handle_projectiles(
    mut projectile_query: Query<(&Projectile, &mut Transform, &mut Velocity)>,
    time: Res<Time>,
) {
    let delta = time.delta_seconds();

    for (projectile, mut transform, mut velocity) in projectile_query.iter_mut() {
        transform.rotation = Quat::from_rotation_arc_2d(Vec2::Y, projectile.direction);

        velocity.linvel = projectile.direction * projectile.stats.speed * delta;
    }
}

fn projectile_spawn_event(
    mut commands: Commands,
    game_assets: Res<GameAssets>,
    mut event_reader: EventReader<SpawnProjectileEvent>,
) {
    for event in event_reader.read() {
        spawn_projectile(
            &mut commands,
            &game_assets,
            event.start_position,
            event.direction,
            event.projectile_stats,
        );
    }
}
