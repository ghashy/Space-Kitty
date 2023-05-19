use bevy::{prelude::*, window::WindowResolution};
use bevy_rapier2d::prelude::*;

// ───── Modules ──────────────────────────────────────────────────────────── //

// Modules in folders
pub mod audio_system;
pub mod game;
pub mod main_menu;

// Top-level modules
mod debug;
pub mod events;
pub mod helper_functions;
mod systems;

use debug::DebugPlugin;
use game::GamePlugin;
use main_menu::MainMenuPlugin;
use systems::*;

// ───── Body ─────────────────────────────────────────────────────────────── //

fn main() {
    App::new()
        .add_startup_system(setup)
        .add_startup_system(spawn_camera)
        .add_state::<AppState>()
        // Plugins
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: String::from("Balls"),
                resolution: WindowResolution::new(1280., 720.),
                ..default()
            }),
            ..default()
        }))
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(200.))
        .add_plugin(GamePlugin)
        .add_plugin(MainMenuPlugin)
        .add_plugin(DebugPlugin)
        // Systems
        .add_system(transition_to_game_state)
        .add_system(transition_to_main_menu_state)
        .add_system(handle_game_over)
        .add_system(exit_game)
        .run();
}

#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    Game,
    GameOver,
}
