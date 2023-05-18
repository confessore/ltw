use crate::Colony;

use bevy::prelude::*;

pub fn step(mut colony: ResMut<Colony>) {
    if colony.current_ores != colony.max_ores {
        colony.current_ores += colony.ores_rate;
        if colony.current_ores > colony.max_ores {
            colony.current_ores = colony.max_ores;
        }
        println!("{} ores in the colony", &colony.current_ores);
    }
    if colony.current_ingots != colony.max_ingots {
        colony.current_ingots += colony.ingots_rate;
        if colony.current_ingots > colony.max_ingots {
            colony.current_ingots = colony.max_ingots;
        }
        println!("{} ingots in the colony", &colony.current_ingots);
    }
    if colony.current_logs != colony.max_logs {
        colony.current_logs += colony.logs_rate;
        if colony.current_logs > colony.max_logs {
            colony.current_logs = colony.max_logs;
        }
        println!("{} logs in the colony", colony.current_logs);
    }
}

pub fn initialize(mut colony: ResMut<Colony>) {
    colony.max_ores = 50.0_f64;
    colony.max_ingots = 50.0_f64;
    colony.max_logs = 50.0_f64;
    colony.ores_rate = 0.1_f64;
    colony.ingots_rate = 0.01_f64;
    colony.logs_rate = 1.0_f64;
}
