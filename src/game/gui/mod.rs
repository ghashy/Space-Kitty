use bevy::prelude::*;
use bevy_tweening::TweenCompleted;

// ───── Current Crate Imports ────────────────────────────────────────────── //

use self::systems::*;
use crate::AppState;

// ───── Submodules ───────────────────────────────────────────────────────── //

// Top-level modules
mod animation;
mod components;
mod styles;
mod systems;

// ───── Body ─────────────────────────────────────────────────────────────── //

pub struct GameUiPlugin;

impl Plugin for GameUiPlugin {
    fn build(&self, app: &mut App) {
        app
            // Events
            .add_event::<TweenCompleted>()
            // States
            .add_state::<HudState>()
            // Enter State Systems
            .add_system(spawn_hud.in_schedule(OnEnter(AppState::Game)))
            // Update Hud State
            .add_system(
                listen_events
                    .in_set(OnUpdate(AppState::Game))
                    .in_set(OnUpdate(HudState::Idle)),
            )
            .add_system(update_hud.in_set(OnUpdate(HudState::Update)))
            // Exit State Systems
            .add_system(despawn_hud.in_schedule(OnExit(AppState::Game)));
    }
}

#[derive(States, Clone, Copy, Hash, PartialEq, Eq, Default, Debug)]
pub enum HudState {
    #[default]
    Idle,
    Update,
}
