use bevy::prelude::*;

// ───── Current Crate Imports ────────────────────────────────────────────── //

use self::{resources::*, systems::*};
use crate::AppState;

// ───── Submodules ───────────────────────────────────────────────────────── //

pub mod resources;
pub mod systems;

// ───── Body ─────────────────────────────────────────────────────────────── //

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app
            // Resources
            .init_resource::<HighScores>()
            // Enter State Systems
            .add_system(insert_score.in_schedule(OnEnter(AppState::Game)))
            // Systems
            .add_system(update_highscores)
            .add_system(high_scores_updated)
            .add_system(update_score.in_set(OnUpdate(AppState::Game)))
            // Exit State Systems
            .add_system(remove_score.in_schedule(OnExit(AppState::Game)));
    }
}
