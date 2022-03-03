use crate::{
    Game,
    Tile,
    BOARD_SIZE_X,
    BOARD_SIZE_Y,
    RESET_FOCUS
};
use bevy::prelude::*;
use bevy_mod_picking::{
    PickableBundle,
    PickingCameraBundle
};

pub fn setup_cameras(
    mut commands: Commands,
    mut game: ResMut<Game>) {
        game.camera_to = Vec3::from(RESET_FOCUS);
        game.camera_from = game.camera_to;
        commands.spawn_bundle(PerspectiveCameraBundle {
            transform: Transform::from_xyz(
                -(BOARD_SIZE_X as f32 / 2.0),
                2.0 * BOARD_SIZE_Y as f32 / 5.0,
                BOARD_SIZE_Y as f32 / 2.0
            )
            .looking_at(game.camera_from, Vec3::Y),
            ..Default::default()
        })
        .insert_bundle(PickingCameraBundle::default());
        commands.spawn_bundle(UiCameraBundle::default());
}

pub fn setup(
    mut commands: Commands,
    mut game: ResMut<Game>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut scenes: ResMut<Assets<Scene>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>) {
        commands.spawn_bundle(PointLightBundle {
            point_light: PointLight {
                color: Color::rgb(0.9, 0.9, 0.9),
                intensity: 500.0,
                range: 50.0,
                radius: 0.0,
                shadows_enabled: false,
                ..Default::default()
            },
            transform: Transform::from_xyz(BOARD_SIZE_X as f32 / 2.0, 5.0, BOARD_SIZE_Y as f32 / 2.0),
            ..Default::default()
        });
        let tile_mesh = meshes.add(Mesh::from(shape::Plane {
            size: 1.0 
        }));
        let white_material = materials.add(Color::rgb(1.0, 0.9, 0.9).into());
        let black_material = materials.add(Color::rgb(0.0, 0.1, 0.1).into());
        game.map = (0..BOARD_SIZE_Y).map(|y| {
            (0..BOARD_SIZE_X).map(|x| {
                //let height = rand::thread_rng().gen_range(-10.0..10.0);
                commands.spawn_bundle(PbrBundle {
                    transform: Transform::from_xyz(x as f32, 0.0, y as f32),
                    ..Default::default()
                })
                .with_children(|tile| {
                    tile.spawn_bundle(PbrBundle {
                        mesh: tile_mesh.clone(),
                        material: {
                            if (x + y + 1) % 2 == 0 {
                                black_material.clone()
                            } else {
                                black_material.clone()
                            }
                        },
                        ..Default::default()
                    })
                    .insert_bundle(PickableBundle::default());
                })
                .insert_bundle(PickableBundle::default());
                Tile { 
                    height: 0.0
                }
            })
            .collect()
        })
        .collect();
        game.unit.x = BOARD_SIZE_X / 2;
        game.unit.y = BOARD_SIZE_Y / 2;
        let character = asset_server.load("models/character.glb#Scene0");
        let tree = asset_server.load("models/character.glb#Scene0");
        //let tree_scene = scenes.get_mut(tree).unwrap();
        game.unit.entity = Some(
            commands.spawn_bundle(PbrBundle {
                transform: Transform::from_xyz(BOARD_SIZE_X as f32 / 2.0, 2.5, BOARD_SIZE_Y as f32 / 2.0),
                ..Default::default()
            })
            .with_children(|unit| {
                unit.spawn_scene(character);
            })
            .insert_bundle(PickableBundle::default())
            .id()
        );
        commands.spawn_bundle(PbrBundle {
            transform: Transform::from_xyz(BOARD_SIZE_X as f32 / 4.0, 2.5, BOARD_SIZE_Y as f32 / 4.0),
            ..Default::default()
        })
        .with_children(|unit| {
            unit.spawn_scene(tree);
        })
        .insert_bundle(PickableBundle::default());
}

pub struct PlayingMaterials {
    pub tile_mesh: Handle<Mesh>,
    pub white: Handle<StandardMaterial>,
    pub black: Handle<StandardMaterial>,
    pub character: Handle<Scene>
}

impl PlayingMaterials {
    fn new(
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<StandardMaterial>>,
        asset_server: Res<AssetServer>) -> Self {
        PlayingMaterials {
            tile_mesh: meshes.add(Mesh::from(shape::Plane {
                size: 1.0 
            })),
            white: materials.add(Color::rgb(1.0, 0.9, 0.9).into()),
            black: materials.add(Color::rgb(0.0, 0.1, 0.1).into()),
            character: asset_server.load("models/character.glb#Scene0")
        }
    }
}
