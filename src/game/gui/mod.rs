use bevy::prelude::*;

// ───── Current Crate Imports ────────────────────────────────────────────── //

use self::systems::*;
use crate::AppState;

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
            .add_system(spawn_hud.in_schedule(OnEnter(AppState::Game)))
            // Systems
            .add_systems(
                (
                    update_messages,
                    listen_hit_events,
                    listen_regeneration_events,
                    spawn_rows_from_backend,
                )
                    .in_set(OnUpdate(AppState::Game)),
            )
            .add_system(
                remove_message_on_timeout.in_set(OnUpdate(AppState::Game)),
            )
            // Exit State Systems
            .add_system(despawn_hud.in_schedule(OnExit(AppState::Game)));
    }
}
