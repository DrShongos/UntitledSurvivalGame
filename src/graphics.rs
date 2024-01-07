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
}

fn load_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    let humanoid = asset_server.load("humanoid.png");
    let slash = asset_server.load("slashNormal.png");

    commands.insert_resource(GameAssets { humanoid, slash });
}
