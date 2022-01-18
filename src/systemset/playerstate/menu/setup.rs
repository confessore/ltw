use crate::{
    Game,
    GameState,
    PlayerState
};

use bevy::prelude::*;

pub fn setup(
    mut commands: Commands,
    mut game: ResMut<Game>,
    mut game_state: ResMut<State<GameState>>,
    mut player_state: ResMut<State<PlayerState>>,
    mut transforms: Query<&mut Transform>,
    keyboard_input: Res<Input<KeyCode>>) {
        if keyboard_input.just_pressed(KeyCode::Escape) {
            let state = player_state.current();
            match *state {
                PlayerState::Menu => {
                    player_state.set(PlayerState::Default).unwrap();
                },
                _ => {
                    player_state.set(PlayerState::Menu).unwrap();
                }
            }
        }
}