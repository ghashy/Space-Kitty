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
            .add_system(update_score.run_if(in_state(AppState::Game)))
            .add_system(update_highscores)
            .add_system(high_scores_updated)
            // Exit State Systems
            .add_system(remove_score.in_schedule(OnExit(AppState::Game)));
    }
}
