use crate::{
    Game,
    RESET_FOCUS,
    SPEED
};

use bevy::{
    prelude::*,
    render::{
        camera::{
            Camera,
            CameraPlugin
        }
    }
};

pub fn focus_camera(mut game: ResMut<Game>,
    mut transforms: QuerySet<(
        QueryState<(&mut Transform, &Camera)>,
        QueryState<&Transform>)>,
    time: Res<Time>) {
        if let Some(unit_entity) =  game.unit.entity {
            if let Ok(unit_transform) = transforms.q1().get(unit_entity) {
                game.camera_to = unit_transform.translation;
            } else {
                game.camera_to = Vec3::from(RESET_FOCUS);
            }
        }
        let mut camera_motion = game.camera_to - game.camera_from;
        if camera_motion.length() > 0.2 {
            camera_motion *= SPEED * time.delta_seconds();
            game.camera_from += camera_motion;
        }
        for (mut transform, camera) in transforms.q0().iter_mut() {
            if camera.name == Some(CameraPlugin::CAMERA_3D.to_string()) {
                *transform = transform.looking_at(game.camera_from, Vec3::Y);
            }
        }
}