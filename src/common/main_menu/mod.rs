use bevy::prelude::*;

// ───── Current Crate Imports ────────────────────────────────────────────── //

use self::systems::{
    interactions::*, layout::*, play_title_theme, stop_title_theme,
};
use crate::common::AppState;

// ───── Submodules ───────────────────────────────────────────────────────── //

mod animation;
pub mod components;
mod styles;
pub mod systems;

// ───── Body ─────────────────────────────────────────────────────────────── //

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            // Entery State Systems
            .add_systems(
                OnEnter(AppState::MainMenu),
                (spawn_main_menu, play_title_theme),
            )
            // Interaction Systems
            .add_systems(
                Update,
                (interact_with_play_button, interact_with_quit_button)
                    .run_if(in_state(AppState::MainMenu)),
            )
            // Exit State Systems
            .add_systems(
                OnExit(AppState::MainMenu),
                (despawn_main_menu, stop_title_theme),
            );
    }
}
