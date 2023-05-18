use super::*;
use bevy::prelude::*;

#[derive(Default, Resource)]
pub struct Colony {
    pub ores_rate: f64,
    pub current_ores: f64,
    pub max_ores: f64,

    pub ingots_rate: f64,
    pub current_ingots: f64,
    pub max_ingots: f64,

    pub logs_rate: f64,
    pub current_logs: f64,
    pub max_logs: f64,
}

impl Colony {}
