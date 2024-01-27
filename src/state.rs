use bevy::prelude::*;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Reflect, States)]
pub enum GameState {
    MainMenu,
    LoadingAssets,
    InGame,
    Paused,
}

impl Default for GameState {
    fn default() -> Self {
        // Main Menu not implemented yet
        GameState::LoadingAssets
    }
}
