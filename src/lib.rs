// #![feature(derive_default_enum)] // requires nightly
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

use structopt::StructOpt;

#[derive(StructOpt)]
pub struct Opt {
    #[structopt(short, long)]
    local_port: u16,
    #[structopt(short, long)]
    players: Vec<String>,
    #[structopt(short, long)]
    spectators: Vec<std::net::SocketAddr>,
}

pub const SPEED: f32 = 2.0;

pub const BOARD_SIZE_X: usize = 32;
pub const BOARD_SIZE_Y: usize = 32;

pub const RESET_FOCUS: [f32; 3] = [
    BOARD_SIZE_X as f32 / 2.0,
    0.0,
    BOARD_SIZE_Y as f32 / 2.0,
];