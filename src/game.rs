use super::*;
use bevy::math::Vec3;

#[derive(Default)]
pub struct Game {
    pub map: Vec<Vec<Tile>>,
    pub players: Vec<Player>,
    pub turn_order: Vec<u32>,
    pub camera_from: Vec3,
    pub camera_to: Vec3,
    pub unit: Unit
}

impl Game {

}