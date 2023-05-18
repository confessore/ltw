use bevy::{
    ecs::schedule::SystemSet,
    prelude::*,
    render::camera::{Camera, CameraPlugin},
    window::WindowMode,
};
use ltw::{
    Colony, Game, GameState, PlayerState, RESET_FOCUS, SPEED, systemset::gamestate,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    //let opt = Opt::from_args();
    //let players = opt.players.len();
    //assert!(players > 0);
    //let mut p2p_session = P2PSession::new(players, )
    App::new()
        .add_plugins(DefaultPlugins)
        //.add_plugin(GGRSPlugin)
        .add_state::<GameState>()
        .add_state::<PlayerState>()
        .init_resource::<Game>()
        .init_resource::<Colony>()
        //.add_startup_system(setup_cameras)
        .add_systems(
            OnEnter(GameState::Default),
            gamestate::default::setup::setup,
        )
        .add_systems(
            OnEnter(GameState::Playing),
            (
                gamestate::playing::colony::initialize,
                gamestate::playing::setup::setup_cameras,
                gamestate::playing::setup::setup,
            ),
        )
        .add_systems(
            Update,
            gamestate::playing::colony::step.run_if(in_state(GameState::Playing)),
        )
        .add_systems(
            Update,
            gamestate::playing::movement::move_unit.run_if(in_state(GameState::Playing)),
        )
        .add_systems(
            Update,
            gamestate::playing::camera::focus_camera.run_if(in_state(GameState::Playing)),
        )
        //.add_plugin(WgpuResourceDiagnosticsPlugin::default())
        //.add_system_set(
        //    SystemSet::on_enter(PlayerState::Menu)
        //        .with_system(lol))
        //.add_system(bevy::input::system::exit_on_esc_system)
        .run();
    Ok(())
}
