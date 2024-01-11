use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::prelude::*;

use crate::{character::npc::spawn_friendly, graphics::GameAssets};

pub struct WorldPlugin;

pub const MIN_WORLD_X: f32 = -1000.0;
pub const MIN_WORLD_Y: f32 = -1000.0;

pub const MAX_WORLD_X: f32 = 1000.0;
pub const MAX_WORLD_Y: f32 = 1000.0;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, populate_world);
    }
}

fn populate_world(mut commands: Commands, game_assets: Res<GameAssets>) {
    let mut rng = rand::thread_rng();

    for _ in 0..40 {
        let pos_x = rng.gen_range(MIN_WORLD_X..MAX_WORLD_X) as f32;
        let pos_y = rng.gen_range(MIN_WORLD_Y..MAX_WORLD_Y) as f32;

        spawn_friendly(&mut commands, &game_assets, Vec2::new(pos_x, pos_y));
    }

    for i in 0..20 {
        let pos_x = rng.gen_range(MIN_WORLD_X..MAX_WORLD_X) as f32;
        let pos_y = rng.gen_range(MIN_WORLD_Y..MAX_WORLD_Y) as f32;

        spawn_tree(
            &mut commands,
            &game_assets,
            Vec2::new(pos_x, pos_y),
            i as f32,
        );
    }
}

fn spawn_tree(
    commands: &mut Commands,
    game_assets: &Res<GameAssets>,
    position: Vec2,
    z_offset: f32,
) {
    let root = commands
        .spawn(RigidBody::Fixed)
        .insert(Collider::cuboid(52.0, 2.0))
        .insert(GlobalTransform::default())
        .insert(Transform::from_xyz(-20.0, -192.0, 2.0 + z_offset))
        .id();

    commands
        .spawn(SpriteSheetBundle {
            sprite: TextureAtlasSprite {
                index: 0,
                custom_size: Some(Vec2::new(384.0, 384.0)),
                ..Default::default()
            },
            texture_atlas: game_assets.tree_atlas.clone(),
            transform: Transform::from_translation(position.extend(2.0 + z_offset)),
            ..Default::default()
        })
        .push_children(&[root]);
}
