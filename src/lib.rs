#![feature(derive_default_enum)]
pub mod action;
pub mod button;
pub mod damage;
pub mod effect;
pub mod game;
pub mod player;
pub mod state;
pub mod systemset;
pub mod tile;
pub mod unit;
pub use crate::{
    action::{
        Action,
        InputAction
    },
    button::{
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
    systemset::{
        gamestate::{
        },
        playerstate::{

        }
    },
    tile::{
        Tile
    },
    unit::{
        Unit
    }
};
