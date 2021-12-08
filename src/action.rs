use super::*;
use bevy::math::Vec3;

pub struct Action {
    pub input: InputAction,
    pub targets: Vec<Vec3>,
    pub points: u32,
    pub magical: MagicalDamage,
    pub physical: PhysicalDamage,
    pub damage: u32
}

pub enum InputAction {
    Default,
    Ability,
    Attack,
    Item,
    Move
}
