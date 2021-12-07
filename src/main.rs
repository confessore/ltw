use bevy::{
    ecs::schedule::SystemSet,
    prelude::*,
    window::WindowMode
};
use ltw::{
    Game,
    GameState,
    Tile
};
use rand::Rng;

fn main() {
    App::new()
        .insert_resource(Msaa {
            samples: 4
        })
        .insert_resource(WindowDescriptor {
            title: String::from("ltw"),
            mode: WindowMode::BorderlessFullscreen,
            ..Default::default()
        })
        .init_resource::<Game>()
        .add_plugins(DefaultPlugins)
        .add_state(GameState::Default)
        .add_startup_system(setup_cameras)
        .add_system_set(SystemSet::on_enter(GameState::Default).with_system(setup))
        .run();
}

fn setup_cameras(
    mut commands: Commands,
    mut game: ResMut<Game>) {
        game.camera_to = Vec3::from(RESET_FOCUS);
        game.camera_from = game.camera_to;
        commands.spawn_bundle(PerspectiveCameraBundle {
            transform: Transform::from_xyz(
                -(BOARD_SIZE_X as f32 / 2.0),
                2.0 * BOARD_SIZE_Y as f32 / 3.0,
                BOARD_SIZE_Y as f32 / 2.0 - 0.5
            )
            .looking_at(game.camera_from, Vec3::Y),
            ..Default::default()
        });
        commands.spawn_bundle(UiCameraBundle::default());
}

fn setup(
    mut commands: Commands,
    mut game: ResMut<Game>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>) {
        commands.spawn_bundle(PointLightBundle {
            point_light: PointLight {
                color: Color::rgb(0.9, 0.9, 0.9),
                intensity: 500.0,
                range: 50.0,
                radius: 0.0
            },
            transform: Transform::from_xyz(BOARD_SIZE_X as f32 / 2.0, 5.0, BOARD_SIZE_Y as f32 / 2.0),
            ..Default::default()
        });
        let tile_mesh = meshes.add(Mesh::from(shape::Plane {
            size: 1.0 
        }));
        let white_material = materials.add(Color::rgb(1.0, 0.9, 0.9).into());
        let black_material = materials.add(Color::rgb(0.0, 0.1, 0.1).into());
        game.map = 
            (0..BOARD_SIZE_Y).map(|y| {
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
                                    white_material.clone()
                                } else {
                                    black_material.clone()
                                }
                            },
                            ..Default::default()
                        });
                    });
                    Tile { 
                        height: 0.0
                    }
                })
                .collect()
            })
            .collect()
}

const BOARD_SIZE_X: usize = 32;
const BOARD_SIZE_Y: usize = 32;

const RESET_FOCUS: [f32; 3] = [
    BOARD_SIZE_X as f32 / 2.0,
    0.0,
    BOARD_SIZE_Y as f32 / 2.0 - 0.5,
];
