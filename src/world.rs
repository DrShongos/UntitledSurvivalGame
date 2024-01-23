use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::prelude::*;

use crate::{
    animation::WobbleBundle,
    character::npc::{spawn_npc, NpcKind},
    graphics::GameAssets,
};

pub struct WorldPlugin;

pub const MIN_WORLD_X: f32 = -2000.0;
pub const MIN_WORLD_Y: f32 = -2000.0;

pub const MAX_WORLD_X: f32 = 2000.0;
pub const MAX_WORLD_Y: f32 = 2000.0;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, prepare_world);
    }
}

fn prepare_world(mut commands: Commands, game_assets: Res<GameAssets>) {
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
        .insert(Name::new("Bottom Barrier"));

    for _ in 0..40 {
        let pos_x = rng.gen_range(MIN_WORLD_X..MAX_WORLD_X) as f32;
        let pos_y = rng.gen_range(MIN_WORLD_Y..MAX_WORLD_Y) as f32;

        let npc_kind = if rng.gen_bool(0.5) {
            NpcKind::Friendly
        } else {
            NpcKind::Hostile
        };

        spawn_npc(
            &mut commands,
            &game_assets,
            Vec2::new(pos_x, pos_y),
            npc_kind,
        );
    }

    for i in 0..20 {
        let tree_index = rng.gen_range(0..=3) as usize;
        let pos_x = rng.gen_range(MIN_WORLD_X..MAX_WORLD_X) as f32;
        let pos_y = rng.gen_range(MIN_WORLD_Y..MAX_WORLD_Y) as f32;

        spawn_tree(
            &mut commands,
            &game_assets,
            tree_index,
            Vec2::new(pos_x, pos_y),
            i as f32,
        );
    }

    for _ in 0..60 {
        let rock_index = rng.gen_range(0..=3) as usize;
        let pos_x = rng.gen_range(MIN_WORLD_X..MAX_WORLD_X) as f32;
        let pos_y = rng.gen_range(MIN_WORLD_Y..MAX_WORLD_Y) as f32;

        spawn_rock(
            &mut commands,
            &game_assets,
            rock_index,
            Vec2::new(pos_x, pos_y),
        );
    }
}

fn spawn_tree(
    commands: &mut Commands,
    game_assets: &Res<GameAssets>,
    sprite_index: usize,
    position: Vec2,
    z_offset: f32,
) {
    let tree = commands
        .spawn(SpriteSheetBundle {
            sprite: TextureAtlasSprite {
                index: sprite_index,
                custom_size: Some(Vec2::new(384.0, 384.0)),
                ..Default::default()
            },
            texture_atlas: game_assets.tree_atlas.clone(),
            transform: Transform::from_xyz(20.0, 184.0, 2.0 + z_offset.abs()),
            global_transform: GlobalTransform::default(),
            ..Default::default()
        })
        .insert(WobbleBundle::new(Vec3::ONE))
        .id();

    commands
        .spawn(RigidBody::Fixed)
        .insert(Collider::cuboid(52.0, 2.0))
        .insert(GlobalTransform::default())
        .insert(Transform::from_translation(
            position.extend(2.0_f32.max(2.0 + z_offset)),
        ))
        .insert(InheritedVisibility::default())
        .push_children(&[tree]);
}

fn spawn_rock(
    commands: &mut Commands,
    game_assets: &Res<GameAssets>,
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
            texture_atlas: game_assets.rock_atlas.clone(),
            transform: Transform::from_translation(position.extend(-2.0)),
            global_transform: GlobalTransform::default(),
            ..Default::default()
        })
        .insert(WobbleBundle::new(Vec3::ONE));
}
