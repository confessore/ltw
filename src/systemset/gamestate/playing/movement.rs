use crate::{
    Game,
    BOARD_SIZE_X,
    BOARD_SIZE_Y
};

use bevy::prelude::*;

pub fn move_unit(
    mut commands: Commands,
    mut game: ResMut<Game>,
    mut transforms: Query<&mut Transform>,
    keyboard_input: Res<Input<KeyCode>>) {
        let mut moved = false;
        let mut rotation = 0.0;
        if keyboard_input.pressed(KeyCode::W) {
            if game.unit.x < BOARD_SIZE_X - 1 {
                game.unit.x += 1;
            }
            rotation = std::f32::consts::FRAC_PI_2;
            moved = true;
        }
        if keyboard_input.pressed(KeyCode::A) {
            if game.unit.y > 0 {
                game.unit.y -= 1;
            }
            rotation = std::f32::consts::PI;
            moved = true;
        }
        if keyboard_input.pressed(KeyCode::S) {
            if game.unit.x > 0 {
                game.unit.x -= 1;
            }
            rotation = -std::f32::consts::FRAC_PI_2;
            moved = true;
        }
        if keyboard_input.pressed(KeyCode::D) {
            if game.unit.y < BOARD_SIZE_Y -1 {
                game.unit.y += 1;
            }
            rotation = 0.0;
            moved = true;
        }
        if moved {
            *transforms.get_mut(game.unit.entity.unwrap()).unwrap() = Transform {
                translation: Vec3::new(
                    game.unit.x as f32,
                    2.5,
                    game.unit.y as f32
                ),
                rotation: Quat::from_rotation_y(rotation),
                ..Default::default()
            };
        }
}