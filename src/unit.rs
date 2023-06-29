use bevy::prelude::*;

use crate::job::Job;

#[derive(Default)]
pub struct Unit {
    pub entity: Option<Entity>,

    pub current_health: f64,
    pub max_health: f64,
    pub current_magic: f64,
    pub max_magic: f64,
    pub magical_armor: f64,
    pub physical_armor: f64,

    pub constitution: f64,
    pub agility: f64,
    pub intelligence: f64,
    pub strength: f64,

    pub initiative: f64,

    pub burrowed: bool,
    pub cloaked: bool,
    pub flying: bool,
    pub harvestable: bool,
    pub immune: bool,
    pub selectable: bool,
    pub targetable: bool,

    pub movement: f64,
    pub jump: f64,
    pub actions: f64,

    pub x: usize,
    pub y: usize,
    pub z: usize,

    pub job: Job,
}
