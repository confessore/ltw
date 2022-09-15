use super::*;
use bevy::prelude::*;

#[derive(Default, Resource)]
pub struct Colony {
    pub current_ore: i64,
    pub max_ore: i64,
    pub current_ingot: i64,
    pub max_ingot: i64,
    pub current_logs: i64,
    pub max_logs: i64
}

impl Colony {

}
