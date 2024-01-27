use bevy::{
    asset::{LoadedFolder, RecursiveDependencyLoadState},
    prelude::*,
    utils::HashMap,
};

use crate::state::GameState;

use self::npc::{NpcData, NpcDataLoader};

pub mod npc;

pub trait LoadEntity {
    type ExtraData;

    fn load_entity(
        &self,
        commands: &mut Commands,
        game_sprites: &mut ResMut<GameSprites>,
        asset_server: &Res<AssetServer>,
        additional: &Self::ExtraData,
    );
}

pub struct AssetPlugin;

impl Plugin for AssetPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<NpcData>()
            .init_asset_loader::<NpcDataLoader>()
            .add_systems(OnEnter(GameState::LoadingAssets), prepare_assets)
            .add_systems(
                Update,
                check_loading.run_if(in_state(GameState::LoadingAssets)),
            );
    }
}

#[derive(Resource)]
pub struct GameSprites {
    pub sprites: HashMap<String, Handle<Image>>,
}

#[derive(Resource)]
pub struct EnvironmentAssets {
    pub tree_atlas: Handle<TextureAtlas>,
    pub rock_atlas: Handle<TextureAtlas>,
}

#[derive(Resource)]
pub struct FolderTracker(Handle<LoadedFolder>);

impl GameSprites {
    pub fn get_or_load(
        &mut self,
        sprite_name: &String,
        asset_server: &Res<AssetServer>,
    ) -> Handle<Image> {
        let image = self.sprites.get(sprite_name);

        if image.is_none() {
            let image = asset_server.load(sprite_name);
            self.sprites.insert(sprite_name.clone(), image.clone());

            return image;
        }

        image.unwrap().clone()
    }
}

fn prepare_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let mut game_sprites = GameSprites {
        sprites: HashMap::new(),
    };

    game_sprites.get_or_load(&"human-normal.png".to_string(), &asset_server);
    game_sprites.get_or_load(&"slashNormal.png".to_string(), &asset_server);

    let tree_image = asset_server.load("trees.png");
    let tree_atlas = texture_atlases.add(TextureAtlas::from_grid(
        tree_image,
        Vec2::new(32.0, 32.0),
        4,
        1,
        None,
        None,
    ));

    let rock_image = asset_server.load("rocks.png");
    let rock_atlas = texture_atlases.add(TextureAtlas::from_grid(
        rock_image,
        Vec2::new(16.0, 16.0),
        4,
        1,
        None,
        None,
    ));

    commands.insert_resource(EnvironmentAssets {
        tree_atlas,
        rock_atlas,
    });

    let npc_folder = asset_server.load_folder("npcs");
    commands.insert_resource(FolderTracker(npc_folder));

    commands.insert_resource(game_sprites);
}

fn check_loading(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut game_state: ResMut<NextState<GameState>>,
    folder_tracker: Res<FolderTracker>,
) {
    if let Some(load_state) =
        asset_server.get_recursive_dependency_load_state(folder_tracker.0.clone())
    {
        match load_state {
            RecursiveDependencyLoadState::Loaded => {
                commands.remove_resource::<FolderTracker>();
                game_state.set(GameState::InGame);
            }
            _ => {}
        }
    }
}
