use bevy::prelude::*;

#[derive(Default)]
pub struct Unit {
    pub entity: Option<Entity>,

    pub health: f32,
    pub magical_armor: f32,
    pub physical_armor: f32,

    pub constitution: f32,
    pub finesse: f32,
    pub intelligence: f32,
    pub strength: f32,
    
    pub initiative: f32,

    pub burrowed: bool,
    pub cloaked: bool,
    pub flying: bool,
    pub harvestable: bool,
    pub immune: bool,
    pub selectable: bool,
    pub targetable: bool,

    pub movement: f32,
    pub actions: u32,

    pub speed: f32,
    pub x: usize,
    pub y: usize,
    pub z: usize
}
