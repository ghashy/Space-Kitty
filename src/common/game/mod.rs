use bevy::prelude::*;

// ───── Current Crate Imports ────────────────────────────────────────────── //

use self::{
    gui::GameUiPlugin, regeneration::RegenerationPlugin, resources::GameData,
};
use crate::common::{events::GameOver, AppState};
use enemy::EnemyPlugin;
use fish::FishPlugin;
use player::PlayerPlugin;
use score::ScorePlugin;
use systems::*;

// ───── Submodules ───────────────────────────────────────────────────────── //

// Modules in folders
pub mod components;
pub mod enemy;
mod fish;
mod gui;
pub mod player;
mod regeneration;
mod resources;
pub mod score;

// Top-level modules
mod systems;

// ───── Body ─────────────────────────────────────────────────────────────── //

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app
            // Events
            .add_event::<GameOver>()
            // States
            .add_state::<SimulationState>()
            // Resources
            .init_resource::<GameData>()
            // Enter State Systems
            .add_systems(
                OnEnter(AppState::Game),
                (
                    resume_simulation,
                    spawn_world_borders,
                    system_play_main_theme,
                    spawn_controls_sheet,
                ),
            )
            // Plugins
            .add_plugins(EnemyPlugin)
            .add_plugins(PlayerPlugin)
            .add_plugins(FishPlugin)
            .add_plugins(ScorePlugin)
            .add_plugins(GameUiPlugin)
            .add_plugins(RegenerationPlugin)
            // Systems
            .add_systems(
                Update,
                despawn_controls_sheet.run_if(in_state(AppState::Game)),
            )
            .add_systems(
                Update,
                toggle_simulation_on_input_event
                    .run_if(in_state(AppState::Game)),
            )
            .add_systems(
                Update,
                (
                    system_check_main_theme_clock,
                    system_restart_clock,
                    detect_input,
                )
                    .run_if(in_state(AppState::Game)),
            )
            .add_systems(
                Last,
                despawn_controls_sheet.run_if(in_state(AppState::Game)),
            )
            // Exit State Systems
            .add_systems(
                OnExit(AppState::Game),
                (pause_simulation, despawn_borders, stop_main_theme),
            );
    }
}

#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum SimulationState {
    #[default]
    Running,
    Paused,
}
