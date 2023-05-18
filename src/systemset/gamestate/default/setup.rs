use crate::{button, GameState};

use bevy::prelude::*;

pub fn setup(mut next_state: ResMut<NextState<GameState>>,) {
    next_state.set(GameState::Playing);
}
