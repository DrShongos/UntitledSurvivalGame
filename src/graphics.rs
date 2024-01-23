use bevy::prelude::*;

pub struct GraphicsPlugin;

impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, load_assets);
    }
}

#[derive(Resource)]
pub struct GameAssets {
    pub humanoid: Handle<Image>,
    pub slash: Handle<Image>,
    pub tree_atlas: Handle<TextureAtlas>,
    pub rock_atlas: Handle<TextureAtlas>,
}

fn load_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let humanoid = asset_server.load("humanoid.png");
    let slash = asset_server.load("slashNormal.png");

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

    commands.insert_resource(GameAssets {
        humanoid,
        slash,
        tree_atlas,
        rock_atlas,
    });
}
