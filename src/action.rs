use super::*;
use bevy::prelude::*;
use ggrs::{
    GameInput
};

#[derive(Default, Copy, Clone, Reflect, Hash)]
pub struct Action {
    pub input: InputAction,
    pub target_x: usize,
    pub target_y: usize,
    pub target_z: usize,
    pub points: u32,
}

impl Action {
    pub fn action_to_bytes(&self) -> Vec<u8> {
        let mut bytes = vec![0 as u8; std::mem::size_of::<InputAction>()];
        bytes.push(self.input as u8);
        bytes.push(self.target_x as u8);
        bytes.push(self.target_y as u8);
        bytes.push(self.target_z as u8);
        bytes.push(self.points as u8);
        bytes
    }

    pub fn bytes_to_action(
        input: &Res<Vec<GameInput>>, player_index: usize) -> Action {
            Action {
                input: input[player_index].buffer[0].into(),
                target_x: input[player_index].buffer[1] as usize,
                target_y: input[player_index].buffer[2] as usize,
                target_z: input[player_index].buffer[3] as usize,
                points: input[player_index].buffer[3] as u32,
            }
    }
}

#[derive(Default, Copy, Clone, Reflect, Hash)]
pub enum InputAction {
    #[default]
    Default,
    Ability,
    Attack,
    Item,
    Move
}

impl From<u8> for InputAction {
    fn from(input: u8) -> Self {
        match input {
            0x0 => InputAction::Default,
            0x1 => InputAction::Ability,
            0x2 => InputAction::Attack,
            0x3 => InputAction::Item,
            0x4 => InputAction::Move,
            _ => InputAction::default()
        }
    }
}
