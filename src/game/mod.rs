use bevy::prelude::*;

// ----- Modules ------------------------------------------------------------ //

// Modules in folders
pub mod components;
pub mod enemy;
pub mod player;
mod score;
mod star;

// Top-level modules
mod systems;

// ----- Crate -------------------------------------------------------------- //

use crate::{events::GameOver, AppState};
use enemy::EnemyPlugin;
use player::PlayerPlugin;
use score::ScorePlugin;
use star::StarsPlugin;
use systems::*;

// ----- Body --------------------------------------------------------------- //

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
                (spawn_background, resume_simulation, spawn_world_borders)
                    .in_schedule(OnEnter(AppState::Game)),
            )
            // Plugins
            .add_plugin(EnemyPlugin)
            .add_plugin(PlayerPlugin)
            .add_plugin(StarsPlugin)
            .add_plugin(ScorePlugin)
            // Systems
            .add_system(
                toggle_simulation_on_input_event
                    .run_if(in_state(AppState::Game)),
            )
            // Exit State Systems
            .add_systems(
                (pause_simulation, despawn_background)
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
