#![feature(derive_default_enum)]
pub mod action;
pub mod damage;
pub mod effect;
pub mod game;
pub mod player;
pub mod state;
pub mod system;
pub mod tile;
pub mod unit;
pub use crate::{
    action::{
        Action,
        InputAction
    },
    damage::{
        MagicalDamage,
        PhysicalDamage
    },
    effect::{

    },
    game::{
        Game
    },
    player::{
        Player
    },
    state::{
        GameState,
        PlayerState
    },
    system::{
        defaultsystem::{
            
        },
        movementsystem::{
            
        }
    },
    tile::{
        Tile
    },
    unit::{
        Unit
    }
};
