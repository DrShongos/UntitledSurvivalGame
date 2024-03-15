use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::prelude::*;

use crate::{
    animation::WobbleBundle,
    asset::{npc::NpcData, EnvironmentAssets, GameSprites, LoadEntity},
    state::GameState,
};

pub struct WorldPlugin;

pub const MIN_WORLD_X: f32 = -2000.0;
pub const MIN_WORLD_Y: f32 = -2000.0;

pub const MAX_WORLD_X: f32 = 2000.0;
pub const MAX_WORLD_Y: f32 = 2000.0;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<WorldManager>()
            .add_event::<ClearWorldEvent>()
            .init_resource::<NpcPool>()
            .add_systems(OnEnter(GameState::PreparingWorld), prepare_world)
            .add_systems(OnEnter(GameState::PreparingNpcs), populate_with_npcs)
            .add_systems(
                Update,
                (
                    npc_spawning.run_if(in_state(GameState::InGame)),
                    clear_world_event,
                ),
            );
    }
}

/// A Marker component that marks an entity as part of the world.
/// Used to clear the game when entering the main menu.
#[derive(Component)]
pub struct WorldObject;

#[derive(Event)]
pub struct ClearWorldEvent;

#[derive(Resource, Reflect)]
pub struct WorldManager {
    spawn_timer: Timer,
    difficulty: f32,
}

#[derive(Resource)]
pub struct NpcPool {
    npcs: Vec<NpcData>,
}

impl Default for NpcPool {
    fn default() -> Self {
        Self { npcs: Vec::new() }
    }
}

pub fn prepare_world(
    mut commands: Commands,
    environment_assets: Res<EnvironmentAssets>,
    npcs: Res<Assets<NpcData>>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    let mut rng = rand::thread_rng();

    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::DARK_GRAY,
                custom_size: Some(Vec2::new(100.0, MAX_WORLD_Y * 2.5)),
                ..Default::default()
            },
            transform: Transform::from_xyz(MIN_WORLD_X - 450.0, 0.0, 1.0),
            ..Default::default()
        })
        .insert(Collider::cuboid(50.0, (MAX_WORLD_Y * 2.5) / 2.0))
        .insert(WorldObject)
        .insert(Name::new("Left Barrier"));

    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::DARK_GRAY,
                custom_size: Some(Vec2::new(100.0, MAX_WORLD_Y * 2.5)),
                ..Default::default()
            },
            transform: Transform::from_xyz(MAX_WORLD_X + 450.0, 0.0, 1.0),
            ..Default::default()
        })
        .insert(Collider::cuboid(50.0, (MAX_WORLD_Y * 2.5) / 2.0))
        .insert(WorldObject)
        .insert(Name::new("Right Barrier"));

    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::DARK_GRAY,
                custom_size: Some(Vec2::new(MAX_WORLD_X * 2.5, 100.0)),
                ..Default::default()
            },
            transform: Transform::from_xyz(0.0, MAX_WORLD_Y + 450.0, 1.0),
            ..Default::default()
        })
        .insert(Collider::cuboid((MAX_WORLD_X * 2.5) / 2.0, 50.0))
        .insert(WorldObject)
        .insert(Name::new("Top Barrier"));

    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::DARK_GRAY,
                custom_size: Some(Vec2::new(MAX_WORLD_X * 2.5, 100.0)),
                ..Default::default()
            },
            transform: Transform::from_xyz(0.0, MIN_WORLD_Y - 450.0, 1.0),
            ..Default::default()
        })
        .insert(Collider::cuboid((MAX_WORLD_X * 2.5) / 2.0, 50.0))
        .insert(WorldObject)
        .insert(Name::new("Bottom Barrier"));

    commands.insert_resource(NpcPool {
        npcs: npcs.iter().map(|(_, npc)| npc.clone()).collect::<Vec<_>>(),
    });

    for _ in 0..20 {
        let tree_index = rng.gen_range(0..=3) as usize;
        let pos_x = rng.gen_range(MIN_WORLD_X..MAX_WORLD_X) as f32;
        let pos_y = rng.gen_range(MIN_WORLD_Y..MAX_WORLD_Y) as f32;

        spawn_tree(
            &mut commands,
            &environment_assets,
            tree_index,
            Vec2::new(pos_x, pos_y),
        );
    }

    for _ in 0..60 {
        let rock_index = rng.gen_range(0..=3) as usize;
        let pos_x = rng.gen_range(MIN_WORLD_X..MAX_WORLD_X) as f32;
        let pos_y = rng.gen_range(MIN_WORLD_Y..MAX_WORLD_Y) as f32;

        spawn_rock(
            &mut commands,
            &environment_assets,
            rock_index,
            Vec2::new(pos_x, pos_y),
        );
    }

    commands.insert_resource(WorldManager {
        spawn_timer: Timer::from_seconds(20.0, TimerMode::Repeating),
        difficulty: 0.0,
    });

    game_state.set(GameState::PreparingNpcs);
}

fn populate_with_npcs(
    mut commands: Commands,
    mut game_sprites: ResMut<GameSprites>,
    npc_pool: Res<NpcPool>,
    asset_server: Res<AssetServer>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    spawn_random_npcs(
        &mut commands,
        &mut game_sprites,
        &asset_server,
        &npc_pool,
        40,
        0.0,
    );
    game_state.set(GameState::InGame);
}

fn spawn_random_npcs(
    commands: &mut Commands,
    game_sprites: &mut ResMut<GameSprites>,
    asset_server: &Res<AssetServer>,
    npc_pool: &Res<NpcPool>,
    amount: usize,
    difficulty: f32,
) {
    let mut rng = rand::thread_rng();

    let available_npcs = npc_pool
        .npcs
        .iter()
        .filter(|npc| {
            if let Some(max_diff) = npc.max_difficulty {
                return difficulty >= npc.min_difficulty && difficulty <= max_diff;
            }

            difficulty >= npc.min_difficulty
        })
        .collect::<Vec<_>>();

    for _ in 0..amount {
        if available_npcs.len() > 0 {
            let index = rng.gen_range(0..available_npcs.len());
            let pos_x = rng.gen_range(MIN_WORLD_X..MAX_WORLD_X) as f32;
            let pos_y = rng.gen_range(MIN_WORLD_Y..MAX_WORLD_Y) as f32;

            available_npcs[index].load_entity(
                commands,
                game_sprites,
                asset_server,
                &Vec2::new(pos_x, pos_y),
            );
        }
    }
}

fn spawn_tree(
    commands: &mut Commands,
    environment_assets: &Res<EnvironmentAssets>,
    sprite_index: usize,
    position: Vec2,
) {
    let tree = commands
        .spawn(SpriteSheetBundle {
            sprite: TextureAtlasSprite {
                index: sprite_index,
                custom_size: Some(Vec2::new(384.0, 384.0)),
                ..Default::default()
            },
            texture_atlas: environment_assets.tree_atlas.clone(),
            transform: Transform::from_xyz(20.0, 184.0, 0.0),
            global_transform: GlobalTransform::default(),
            ..Default::default()
        })
        .insert(WorldObject)
        .insert(WobbleBundle::new(Vec3::ONE))
        .id();

    commands
        .spawn(RigidBody::Fixed)
        .insert(Collider::cuboid(52.0, 2.0))
        .insert(GlobalTransform::default())
        .insert(Transform::from_translation(position.extend(2.0)))
        .insert(WorldObject)
        .insert(InheritedVisibility::default())
        .push_children(&[tree]);
}

fn spawn_rock(
    commands: &mut Commands,
    environment_assets: &Res<EnvironmentAssets>,
    sprite_index: usize,
    position: Vec2,
) {
    commands
        .spawn(SpriteSheetBundle {
            sprite: TextureAtlasSprite {
                index: sprite_index,
                custom_size: Some(Vec2::new(64.0, 64.0)),
                ..Default::default()
            },
            texture_atlas: environment_assets.rock_atlas.clone(),
            transform: Transform::from_translation(position.extend(-2.0)),
            global_transform: GlobalTransform::default(),
            ..Default::default()
        })
        .insert(WorldObject)
        .insert(WobbleBundle::new(Vec3::ONE));
}

fn clear_world_event(
    mut commands: Commands,
    world_object_query: Query<Entity, With<WorldObject>>,
    mut clear_world_event: EventReader<ClearWorldEvent>,
) {
    for _ in clear_world_event.read() {
        for world_entity in world_object_query.iter() {
            commands.entity(world_entity).despawn_recursive();
        }
    }
}

fn npc_spawning(
    mut commands: Commands,
    mut game_sprites: ResMut<GameSprites>,
    npc_pool: Res<NpcPool>,
    asset_server: Res<AssetServer>,
    time: Res<Time>,
    mut world_manager: ResMut<WorldManager>,
) {
    world_manager.spawn_timer.tick(time.delta());
    world_manager.difficulty += 0.0002;

    if world_manager.spawn_timer.just_finished() {
        let mut rng = rand::thread_rng();
        let amount = rng.gen_range(1..=4usize);

        println!(
            "requesting to spawn {amount} entities. current difficulty: {}",
            world_manager.difficulty
        );

        spawn_random_npcs(
            &mut commands,
            &mut game_sprites,
            &asset_server,
            &npc_pool,
            amount,
            world_manager.difficulty,
        );
    }
}
