use bevy::prelude::*;

// ───── Current Crate Imports ────────────────────────────────────────────── //

use self::systems::{interactions::*, layout::*, play_title_theme};
use crate::AppState;

// ───── Submodules ───────────────────────────────────────────────────────── //

mod animation;
mod components;
mod styles;
mod systems;

// ───── Body ─────────────────────────────────────────────────────────────── //

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            // Entery State Systems
            .add_systems(
                (spawn_main_menu, play_title_theme)
                    .in_schedule(OnEnter(AppState::MainMenu)),
            )
            // Interaction Systems
            .add_systems(
                (interact_with_play_button, interact_with_quit_button)
                    .in_set(OnUpdate(AppState::MainMenu)),
            )
            // Exit State Systems
            .add_system(
                despawn_main_menu.in_schedule(OnExit(AppState::MainMenu)),
            );
    }
}
