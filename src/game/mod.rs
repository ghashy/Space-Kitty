use bevy::prelude::*;

// ───── Current Crate Imports ────────────────────────────────────────────── //

use self::{
    gui::GameUiPlugin, regeneration::RegenerationPlugin, resources::GameData,
};
use crate::{events::GameOver, AppState};
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
                (
                    resume_simulation,
                    spawn_world_borders,
                    system_play_main_theme,
                    spawn_controls_sheet,
                )
                    .in_schedule(OnEnter(AppState::Game)),
            )
            // Plugins
            .add_plugin(EnemyPlugin)
            .add_plugin(PlayerPlugin)
            .add_plugin(FishPlugin)
            .add_plugin(ScorePlugin)
            .add_plugin(GameUiPlugin)
            .add_plugin(RegenerationPlugin)
            // Systems
            .add_system(despawn_controls_sheet.in_set(OnUpdate(AppState::Game)))
            .add_system(
                toggle_simulation_on_input_event
                    .run_if(in_state(AppState::Game)),
            )
            .add_systems(
                (
                    system_check_main_theme_clock,
                    system_restart_clock,
                    detect_input,
                )
                    .in_set(OnUpdate(AppState::Game)),
            )
            .add_system(
                despawn_controls_sheet
                    .in_base_set(CoreSet::Last)
                    .run_if(in_state(AppState::Game)),
            )
            // Exit State Systems
            .add_systems(
                (pause_simulation, despawn_borders, stop_main_theme)
                    .in_schedule(OnExit(AppState::Game)),
            );
    }
}

#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum SimulationState {
    #[default]
    Running,
    Paused,
}
