use bevy::{
    ecs::schedule::SystemSet,
    prelude::*,
    render::{
        camera::{
            Camera,
            CameraPlugin
        }
    }
};
use bevy_mod_picking::*;
use ltw::{
    systemset::{
        gamestate,
        playerstate
    },
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
            //mode: WindowMode::BorderlessFullscreen,
            width: 1280.0,
            height: 720.0,
            ..Default::default()
        })

        .add_plugins(DefaultPlugins)
        //.add_plugin(GGRSPlugin)
        .add_plugins(DefaultPickingPlugins)

        .add_state(GameState::Default)
        .add_state(PlayerState::Default)

        .init_resource::<Game>()
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
                .with_system(gamestate::playing::setup::setup_cameras)
                .with_system(gamestate::playing::setup::setup))
        .add_system_set(
            SystemSet::on_update(GameState::Playing)
                .with_system(gamestate::playing::movement::move_unit)
                .with_system(gamestate::playing::camera::focus_camera))
        .add_system_set(
            SystemSet::on_exit(GameState::Playing)
                .with_system(gamestate::playing::teardown::teardown))
        /*.add_system_set(
            SystemSet::on_enter(PlayerState::Default)
                .with_system())
        .add_system_set(
            SystemSet::on_update(PlayerState::Default)
                .with_system())
        .add_system_set(
            SystemSet::on_exit(PlayerState::Default)
                .with_system())
        .add_system_set(
            SystemSet::on_enter(PlayerState::Menu)
                .with_system())
        .add_system_set(
            SystemSet::on_update(PlayerState::Menu)
                .with_system())
        .add_system_set(
            SystemSet::on_exit(PlayerState::Menu)
                .with_system())*/
        //.add_plugin(WgpuResourceDiagnosticsPlugin::default())
        //.add_system_set(
        //    SystemSet::on_enter(PlayerState::Menu)
        //        .with_system(lol))
        //.add_system(bevy::input::system::exit_on_esc_system)
        .run();
    Ok(())
}
