use bevy::prelude::*;

// ───── Current Crate Imports ────────────────────────────────────────────── //

use self::systems::{
    layout::{despawn_gameover_layout, scroll_list, spawn_gameover_layout},
    play_gameover_theme, stop_gameover_theme,
};
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
        app
            // Enter State Systems
            .add_systems(
                (spawn_gameover_layout, play_gameover_theme)
                    .in_schedule(OnEnter(AppState::GameOver)),
            )
            // Systems
            .add_systems((scroll_list,).in_set(OnUpdate(AppState::GameOver)))
            // Exit State Systems
            .add_systems(
                (despawn_gameover_layout, stop_gameover_theme)
                    .in_schedule(OnExit(AppState::GameOver)),
            );
    }
}
