use bevy::{
    asset::{AssetLoader, AsyncReadExt},
    prelude::*,
};
use bevy_rapier2d::prelude::*;
use rand::Rng;
use serde::Deserialize;
use thiserror::Error;

use crate::{
    animation::WobbleBundle,
    character::{
        npc::{NpcController, NpcKind},
        Character, ProjectileShooter,
    },
    combat::{self, Immunity, ProjectileStats, ENEMY_GROUP, PROJECTILE_GROUP},
};

use super::{GameSprites, LoadEntity};

#[derive(Asset, TypePath, Debug, Deserialize)]
pub struct NpcData {
    pub sprite_name: String,
    pub sprite_size: Vec2,
    pub sprite_color: Color,
    pub collider_halfextents: Vec2,

    pub min_difficulty: f32,
    pub max_difficulty: Option<f32>,

    pub max_health: f32,
    pub attack_speed: f32,
    pub projectile_stats: ProjectileData,

    pub speed: f32,
    pub kind: NpcKind,
}

#[derive(Debug, Deserialize)]
pub struct ProjectileData {
    pub damage: f32,
    pub knockback: f32,
    pub speed: f32,
    pub life_time: f32,
}

impl LoadEntity for NpcData {
    type ExtraData = Vec2;

    fn load_entity(
        &self,
        commands: &mut Commands,
        game_sprites: &mut ResMut<GameSprites>,
        asset_server: &Res<AssetServer>,
        additional: &Self::ExtraData,
    ) {
        let npc = commands
            .spawn((
                SpriteBundle {
                    sprite: Sprite {
                        custom_size: Some(self.sprite_size),
                        color: self.sprite_color,
                        ..Default::default()
                    },
                    transform: Transform::from_translation(additional.extend(0.0)),
                    texture: game_sprites.get_or_load(&self.sprite_name, asset_server),
                    ..Default::default()
                },
                RigidBody::Dynamic,
                Collider::cuboid(self.collider_halfextents.x, self.collider_halfextents.y),
                Velocity::zero(),
                LockedAxes::ROTATION_LOCKED,
                CollisionGroups::new(
                    Group::from_bits_truncate(ENEMY_GROUP),
                    Group::from_bits_truncate(PROJECTILE_GROUP | 0b0001),
                ),
                Character {
                    max_health: self.max_health,
                    health: self.max_health,

                    input: Vec2::ZERO,
                    last_x: -1.0,
                    speed: self.speed,
                    accel: 3.9,
                    damp: 5.0,
                },
                WobbleBundle::new(Vec3::ONE),
                Immunity(Timer::from_seconds(0.25, TimerMode::Once)),
                ProjectileShooter {
                    attack_speed: Timer::from_seconds(self.attack_speed, TimerMode::Once),
                    projectile_stats: ProjectileStats {
                        damage: self.projectile_stats.damage,
                        knockback: self.projectile_stats.knockback,
                        speed: self.projectile_stats.speed,
                        life_time: Timer::from_seconds(
                            self.projectile_stats.life_time,
                            TimerMode::Once,
                        ),
                    },
                },
                NpcController {
                    target: None,
                    target_change: Timer::from_seconds(
                        rand::thread_rng().gen_range(5.0..15.0),
                        TimerMode::Repeating,
                    ),
                    kind: self.kind.clone(),
                },
            ))
            .id();

        let healthbar_offset = (-self.collider_halfextents.y) - 20.0;
        combat::healthbar::spawn_healthbar(commands, Vec2::new(0.0, healthbar_offset), npc);
    }
}

#[derive(Default)]
pub struct NpcDataLoader;

#[non_exhaustive]
#[derive(Debug, Error)]
pub enum NpcDataLoaderError {
    #[error("Could not load asset: {0}")]
    Io(#[from] std::io::Error),
    /// A [RON](ron) Error
    #[error("Could not parse RON: {0}")]
    RonSpannedError(#[from] ron::error::SpannedError),
}

impl AssetLoader for NpcDataLoader {
    type Asset = NpcData;
    type Settings = ();
    type Error = NpcDataLoaderError;
    fn load<'a>(
        &'a self,
        reader: &'a mut bevy::asset::io::Reader,
        _settings: &'a Self::Settings,
        _load_context: &'a mut bevy::asset::LoadContext,
    ) -> bevy::utils::BoxedFuture<'a, Result<Self::Asset, Self::Error>> {
        Box::pin(async move {
            let mut bytes = Vec::new();
            reader.read_to_end(&mut bytes).await?;

            let asset = ron::de::from_bytes::<NpcData>(&bytes)?;
            Ok(asset)
        })
    }

    fn extensions(&self) -> &[&str] {
        &["npc"]
    }
}
