use bevy::prelude::*;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Reflect, States)]
pub enum GameState {
    MainMenu,
    LoadingAssets,
    PreparingWorld,
    PreparingNpcs,
    InGame,
    Paused,
    Dead,
}

impl Default for GameState {
    fn default() -> Self {
        GameState::MainMenu
    }
}
