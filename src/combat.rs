use core::panic;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::graphics::GameAssets;

pub struct CombatPlugin;

pub const PROJECTILE_GROUP: u32 = 0b1000;
pub const PLAYER_GROUP: u32 = 0b0010;
pub const ENEMY_GROUP: u32 = 0b0100;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnProjectileEvent>()
            .add_systems(Update, (projectile_spawn_event, collision_event))
            .add_systems(FixedUpdate, handle_projectiles);
    }
}

#[derive(Event)]
pub struct SpawnProjectileEvent {
    pub projectile_stats: ProjectileStats,
    pub direction: Vec2,
    pub start_position: Vec2,
    pub target_group: u32,
}

#[derive(Clone)]
pub struct ProjectileStats {
    pub speed: f32,
    pub life_time: Timer,
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
    target_group: u32,
) {
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(64.0, 48.0)),
                ..Default::default()
            },
            transform: Transform {
                translation: position.extend(1.0),
                rotation: Quat::from_rotation_arc_2d(Vec2::Y, direction),
                ..Default::default()
            },
            texture: game_assets.slash.clone(),
            ..Default::default()
        })
        .insert(RigidBody::Dynamic)
        .insert(Collider::cuboid(32.0, 24.0))
        .insert(Sensor)
        .insert(CollisionGroups::new(
            Group::from_bits_truncate(PROJECTILE_GROUP),
            Group::from_bits_truncate(target_group | 0b0001),
        ))
        .insert(SolverGroups::new(
            Group::from_bits_truncate(PROJECTILE_GROUP),
            Group::from_bits_truncate(target_group | 0b0001),
        ))
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(LockedAxes::ROTATION_LOCKED)
        .insert(Velocity::zero())
        .insert(Projectile {
            stats: projectile_stats,
            direction,
        });
}

fn handle_projectiles(
    mut commands: Commands,
    mut projectile_query: Query<(Entity, &mut Projectile, &mut Transform, &mut Velocity)>,
    time: Res<Time>,
) {
    let delta = time.delta_seconds();

    for (entity, mut projectile, mut transform, mut velocity) in projectile_query.iter_mut() {
        // Maybe it's not a good idea to make the game crash over this
        if projectile.stats.life_time.mode() != TimerMode::Once {
            panic!(
                "Attempted to handle the behaviour of a projectile that has a repeating lifetime"
            );
        }

        projectile.stats.life_time.tick(time.delta());

        if projectile.stats.life_time.finished() {
            commands.entity(entity).despawn_recursive();
        }

        transform.rotation = Quat::from_rotation_arc_2d(Vec2::Y, projectile.direction);

        velocity.linvel = projectile.direction * projectile.stats.speed * delta;
    }
}

fn collision_event(
    mut commands: Commands,
    projectile_query: Query<&Projectile>,
    mut events: EventReader<CollisionEvent>,
) {
    for event in events.read() {
        // The projectile should always be the second entity in the event (hopefully) (please)
        if let CollisionEvent::Started(_, projectile, _) = event {
            if projectile_query.get(*projectile).is_ok() {
                commands.entity(*projectile).despawn_recursive();
            }
        }
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
            event.projectile_stats.clone(),
            event.target_group,
        );
    }
}
