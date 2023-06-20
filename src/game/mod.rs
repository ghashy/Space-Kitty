use bevy::prelude::*;

// ───── Current Crate Imports ────────────────────────────────────────────── //

use crate::{events::GameOver, AppState};
use enemy::EnemyPlugin;
use fish::FishPlugin;
use player::PlayerPlugin;
use score::ScorePlugin;
use systems::*;

use self::gui::GameUiPlugin;

// ───── Submodules ───────────────────────────────────────────────────────── //

// Modules in folders
pub mod components;
pub mod enemy;
mod fish;
mod gui;
pub mod player;
mod score;

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
            // Enter State Systems
            .add_systems(
                (resume_simulation, spawn_world_borders)
                    .in_schedule(OnEnter(AppState::Game)),
            )
            // Plugins
            .add_plugin(EnemyPlugin)
            .add_plugin(PlayerPlugin)
            .add_plugin(FishPlugin)
            .add_plugin(ScorePlugin)
            .add_plugin(GameUiPlugin)
            // Systems
            .add_systems(
                (
                    toggle_simulation_on_input_event,
                    system_camera_follows_player,
                )
                    .in_set(OnUpdate(AppState::Game)),
            )
            // Exit State Systems
            .add_systems(
                (pause_simulation, despawn_borders)
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
