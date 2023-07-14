use bevy::prelude::*;

// ───── Current Crate Imports ────────────────────────────────────────────── //

use self::systems::*;
use crate::common::AppState;

// ───── Submodules ───────────────────────────────────────────────────────── //

// Top-level modules
mod animation;
pub mod components;
mod styles;
mod systems;

// ───── Constants ────────────────────────────────────────────────────────── //

pub const CHART_SIZE: usize = 3;
const HIT_EVENTS_OFFSET: u64 = 400;
const REGEN_EVENTS_OFFSET: u64 = 450;

// ───── Body ─────────────────────────────────────────────────────────────── //

pub struct GameUiPlugin;

impl Plugin for GameUiPlugin {
    fn build(&self, app: &mut App) {
        app
            // Enter State Systems
            .add_systems(OnEnter(AppState::Game), spawn_hud)
            // Systems
            .add_systems(
                Update,
                (
                    update_messages,
                    listen_hit_events,
                    listen_regeneration_events,
                    spawn_rows_from_backend,
                )
                    .run_if(in_state(AppState::Game)),
            )
            .add_systems(
                Update,
                remove_message_on_timeout.run_if(in_state(AppState::Game)),
            )
            // Exit State Systems
            .add_systems(OnExit(AppState::Game), despawn_hud);
    }
}
