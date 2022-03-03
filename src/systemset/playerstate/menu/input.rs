use crate::{
    PlayerState
};

use bevy::prelude::*;

pub fn input(
    mut player_state: ResMut<State<PlayerState>>,
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