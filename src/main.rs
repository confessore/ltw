use bevy::{
    ecs::schedule::SystemSet,
    prelude::*,
    render::{
        camera::{
            Camera,
            CameraPlugin
        }
    },
    window::{
        WindowMode
    }
};
use ltw::{
    systemset::{
        gamestate,
        playerstate
    },
    Colony,
    Game,
    GameState,
    PlayerState,
    RESET_FOCUS,
    SPEED
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    //let opt = Opt::from_args();
    //let players = opt.players.len();
    //assert!(players > 0);
    //let mut p2p_session = P2PSession::new(players, )
    App::new()
        /*.insert_resource(Msaa {
            samples: 4
        })*/
        .insert_resource(WindowDescriptor {
            title: String::from("ltw"),
            mode: WindowMode::Windowed,
            width: 1280.0,
            height: 720.0,
            ..default()
        })

        .add_plugins(DefaultPlugins)
        //.add_plugin(GGRSPlugin)
        
        .add_state(GameState::Default)
        .add_state(PlayerState::Default)

        .init_resource::<Game>()
        .init_resource::<Colony>()
        //.add_startup_system(setup_cameras)
        .add_system_set(
            SystemSet::on_enter(GameState::Default)
                .with_system(gamestate::default::setup::setup))
        .add_system_set(
            SystemSet::on_update(GameState::Default)
                .with_system(gamestate::default::ui::button_system))
        .add_system_set(
            SystemSet::on_exit(GameState::Default)
                .with_system(gamestate::default::teardown::teardown))
        .add_system_set(
            SystemSet::on_enter(GameState::Playing)
                .with_system(gamestate::playing::colony::initialize)
                .with_system(gamestate::playing::setup::setup_ui)
                .with_system(gamestate::playing::setup::setup_cameras)
                .with_system(gamestate::playing::setup::setup))
        .add_system_set(
            SystemSet::on_update(GameState::Playing) 
                //.with_system(gamestate::playing::input::input)
                .with_system(gamestate::playing::colony::step)
                .with_system(gamestate::playing::movement::move_unit)
                .with_system(gamestate::playing::camera::focus_camera)
                .with_system(gamestate::playing::ui::button_system)
                .with_system(gamestate::playing::ui::update_ingots))
        .add_system_set(
            SystemSet::on_exit(GameState::Playing)
                .with_system(gamestate::playing::teardown::teardown))
        .add_system_set(
            SystemSet::on_enter(PlayerState::Default)
                .with_system(playerstate::default::setup::setup))
        .add_system_set(
            SystemSet::on_update(PlayerState::Default)
                .with_system(playerstate::default::input::input))
        .add_system_set(
            SystemSet::on_exit(PlayerState::Default)
                .with_system(playerstate::default::teardown::teardown))
        .add_system_set(
            SystemSet::on_enter(PlayerState::Menu)
                .with_system(playerstate::menu::setup::setup))
        .add_system_set(
            SystemSet::on_update(PlayerState::Menu)
                .with_system(playerstate::menu::input::input))
        .add_system_set(
            SystemSet::on_exit(PlayerState::Menu)
                .with_system(playerstate::menu::teardown::teardown))
        //.add_plugin(WgpuResourceDiagnosticsPlugin::default())
        //.add_system_set(
        //    SystemSet::on_enter(PlayerState::Menu)
        //        .with_system(lol))
        //.add_system(bevy::input::system::exit_on_esc_system)
        .run();
    Ok(())
}
