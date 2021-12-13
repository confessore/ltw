use bevy::{
    ecs::schedule::SystemSet,
    diagnostic::{
        Diagnostics
    },
    prelude::*,
    render::{
        camera::Camera,
        render_graph::base::camera::CAMERA_3D
    },
    window::WindowMode,
    wgpu::{
        diagnostic::{
            WgpuResourceDiagnosticsPlugin
        }
    }
};
use bevy_ggrs::{
    GGRSPlugin
};
use ltw::{
    material::{
        menubuttonmaterial::MenuButtonMaterial
    },
    system::{
        defaultsystem,
        movementsystem
    },
    Game,
    GameState,
    PlayerState,
    Tile
};
use rand::Rng;
use structopt::StructOpt;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    //let opt = Opt::from_args();
    //let players = opt.players.len();
    //assert!(players > 0);
    //let mut p2p_session = P2PSession::new(players, )
    App::new()
        .insert_resource(Msaa {
            samples: 4
        })
        .insert_resource(WindowDescriptor {
            title: String::from("ltw"),
            //mode: WindowMode::BorderlessFullscreen,
            width: 1280.0,
            height: 720.0,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        //.add_plugin(GGRSPlugin)
        .add_state(GameState::Default)
        .add_state(PlayerState::Default)
        .init_resource::<Game>()
        .init_resource::<MenuButtonMaterial>()
        .add_startup_system(setup_cameras)
        .add_system_set(
            SystemSet::on_enter(GameState::Default)
                //.with_system(setup)
                .with_system(defaultsystem::setup))
        .add_system_set(
            SystemSet::on_update(GameState::Default)
                .with_system(defaultsystem::button_system))
                //.with_system(toggle_override)
                //.with_system(change_scale_factor)
                //.with_system(move_unit)
                //.with_system(focus_camera)
                //.with_system(menu))
        //.add_plugin(WgpuResourceDiagnosticsPlugin::default())
        //.add_system_set(
        //    SystemSet::on_enter(PlayerState::Menu)
        //        .with_system(lol))
        //.add_system(bevy::input::system::exit_on_esc_system)
        .run();
    Ok(())
}

/// This system toggles scale factor overrides when enter is pressed
fn toggle_override(input: Res<Input<KeyCode>>, mut windows: ResMut<Windows>) {
    let window = windows.get_primary_mut().unwrap();
    if input.just_pressed(KeyCode::Return) {
        window.set_scale_factor_override(window.scale_factor_override().xor(Some(1.)));
    }
}

/// This system changes the scale factor override when up or down is pressed
fn change_scale_factor(input: Res<Input<KeyCode>>, mut windows: ResMut<Windows>) {
    let window = windows.get_primary_mut().unwrap();
    if input.just_pressed(KeyCode::Up) {
        window.set_scale_factor_override(window.scale_factor_override().map(|n| n + 1.));
    } else if input.just_pressed(KeyCode::Down) {
        window.set_scale_factor_override(window.scale_factor_override().map(|n| (n - 1.).max(1.)));
    }
}

fn setup_cameras(
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
        .collect();
        game.unit.x = BOARD_SIZE_X / 2;
        game.unit.y = BOARD_SIZE_Y / 2;
        let character = asset_server.load("models/character.glb#Scene0");
        game.unit.entity = Some(
            commands.spawn_bundle(PbrBundle {
                transform: Transform::from_xyz(BOARD_SIZE_X as f32 / 2.0, 2.0, BOARD_SIZE_Y as f32 / 2.0),
                ..Default::default()
            })
            .with_children(|unit| {
                unit.spawn_scene(character);
            })
            .id()
        );
}

fn move_unit(
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

fn focus_camera(mut game: ResMut<Game>,
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
            if camera.name == Some(CAMERA_3D.to_string()) {
                *transform = transform.looking_at(game.camera_from, Vec3::Y);
            }
        }
}

fn menu(
    mut commands: Commands,
    mut game: ResMut<Game>,
    mut game_state: ResMut<State<GameState>>,
    mut player_state: ResMut<State<PlayerState>>,
    mut transforms: Query<&mut Transform>,
    keyboard_input: Res<Input<KeyCode>>) {
        if keyboard_input.just_pressed(KeyCode::Escape) {
            let state = player_state.current();
            if *state == PlayerState::Playing {
                player_state.set(PlayerState::Menu).unwrap();
                return;
            }
            else if *state == PlayerState::Menu {
                player_state.set(PlayerState::Playing).unwrap();
                return;
            } else {
                return;
            }
        }
}

fn lol() {
    println!("menu");
}

fn start_p2p_session() {

}

#[derive(StructOpt)]
struct Opt {
    #[structopt(short, long)]
    local_port: u16,
    #[structopt(short, long)]
    players: Vec<String>,
    #[structopt(short, long)]
    spectators: Vec<std::net::SocketAddr>,
}

const SPEED: f32 = 2.0;

const BOARD_SIZE_X: usize = 32;
const BOARD_SIZE_Y: usize = 32;

const RESET_FOCUS: [f32; 3] = [
    BOARD_SIZE_X as f32 / 2.0,
    0.0,
    BOARD_SIZE_Y as f32 / 2.0,
];
