use bevy::prelude::States;

#[derive(Default, Clone, Debug, Eq, Hash, PartialEq, States)]
pub enum GameState {
    #[default]
    Default,
    Playing,
}

#[derive(Default, Clone, Debug, Eq, Hash, PartialEq, States)]
pub enum PlayerState {
    #[default]
    Default,
    Menu,
}
