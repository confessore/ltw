use super::*;
use bevy::prelude::*;

#[derive(Default, Resource)]
pub struct Game {
    pub map: Vec<Vec<Tile>>,
    pub players: Vec<Player>,
    pub turn_order: Vec<u32>,
    pub camera_previous: Vec3,
    pub camera_current: Vec3,
    pub unit: Unit,
}

impl Game {}
