use crate::{Game, RESET_FOCUS, SPEED};

use bevy::prelude::*;

pub fn focus_camera(
    time: Res<Time>,
    mut game: ResMut<Game>,
    mut transforms: ParamSet<(Query<&mut Transform, With<Camera3d>>, Query<&Transform>)>,
) {
    if let Some(unit_entity) = game.unit.entity {
        if let Ok(unit_transform) = transforms.p1().get(unit_entity) {
            game.camera_current = unit_transform.translation;
        } else {
            game.camera_current = Vec3::from(RESET_FOCUS);
        }
    }
    let mut camera_motion = game.camera_current - game.camera_previous;
    if camera_motion.length() > 0.2 {
        camera_motion *= SPEED * time.delta_seconds();
        game.camera_previous += camera_motion;
    }
    for mut transform in transforms.p0().iter_mut() {
        *transform = transform.looking_at(game.camera_current, Vec3::Y);
    }
}
