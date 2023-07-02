use bevy::prelude::*;

// ───── Current Crate Imports ────────────────────────────────────────────── //

use self::systems::layout::{scroll_list, spawn_gameover_layout};
use crate::AppState;

// ───── Submodules ───────────────────────────────────────────────────────── //

// Modules in folders
mod systems;

// Top-level modules
mod components;
mod styles;

// ───── Body ─────────────────────────────────────────────────────────────── //

pub struct GameoverPlugin;

impl Plugin for GameoverPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(
            spawn_gameover_layout.in_schedule(OnEnter(AppState::GameOver)),
        )
        .add_systems((scroll_list,).in_set(OnUpdate(AppState::GameOver)));
    }
}
