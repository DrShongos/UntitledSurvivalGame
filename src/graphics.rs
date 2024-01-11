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
}

fn load_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let humanoid = asset_server.load("humanoid.png");
    let slash = asset_server.load("slashNormal.png");

    let tree_image = asset_server.load("tree-sheet.png");
    let tree_atlas = texture_atlases.add(TextureAtlas::from_grid(
        tree_image,
        Vec2::new(128.0, 128.0),
        2,
        1,
        None,
        None,
    ));

    commands.insert_resource(GameAssets {
        humanoid,
        slash,
        tree_atlas,
    });
}
