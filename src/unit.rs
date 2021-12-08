use bevy::prelude::Entity;

#[derive(Default)]
pub struct Unit {
    pub entity: Option<Entity>,
    pub health: f32,
    pub actions: u32,
    pub speed: f32,
    pub x: usize,
    pub y: usize,
    pub z: usize
}
