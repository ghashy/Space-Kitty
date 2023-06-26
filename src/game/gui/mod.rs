use bevy::prelude::*;

// ───── Current Crate Imports ────────────────────────────────────────────── //

use self::systems::*;
use crate::AppState;

// ───── Submodules ───────────────────────────────────────────────────────── //

// Top-level modules
mod animation;
mod components;
mod styles;
mod systems;

// ───── Constants ────────────────────────────────────────────────────────── //

const LIVES_ID_OFFSET: u64 = 400;
const CHART_SIZE: u32 = 3;

// ───── Body ─────────────────────────────────────────────────────────────── //

pub struct GameUiPlugin;

impl Plugin for GameUiPlugin {
    fn build(&self, app: &mut App) {
        app
            // Enter State Systems
            .add_system(spawn_hud.in_schedule(OnEnter(AppState::Game)))
            // Update Hud State
            .add_system(listen_hit_events.in_set(OnUpdate(AppState::Game)))
            // Systems
            .add_system(update_messages.in_set(OnUpdate(AppState::Game)))
            // .add_system(update_lives.in_set(OnUpdate(HudLivesState::Update)))
            .add_system(
                remove_message_on_timeout.in_set(OnUpdate(AppState::Game)),
            )
            // Exit State Systems
            .add_system(despawn_hud.in_schedule(OnExit(AppState::Game)));
    }
}
