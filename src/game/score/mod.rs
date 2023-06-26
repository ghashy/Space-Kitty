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
            // Events
            .add_event::<ScoreUpdateEvent>()
            // Resources
            .init_resource::<HighScores>()
            .init_resource::<Chart>()
            // Enter State Systems
            .add_system(insert_score.in_schedule(OnEnter(AppState::Game)))
            // Systems
            .add_system(sort_highscores)
            .add_system(
                update_score
                    // IMPORTANT: we should update score before we spawn
                    // enemy on game progress, because spawning depends on
                    // score value.
                    .run_if(in_state(AppState::Game))
                    .in_base_set(CoreSet::First),
            )
            // Exit State Systems
            .add_system(remove_score.in_schedule(OnExit(AppState::Game)));
    }
}

// Events

pub struct ScoreUpdateEvent {
    pub name: String,
    pub event_type: ScoreEventType,
}

pub enum ScoreEventType {
    NewHighscore(u32),
    ReachedMilestone(u32),
}

impl ScoreUpdateEvent {
    pub fn new(name: String, event_type: ScoreEventType) -> Self {
        ScoreUpdateEvent { name, event_type }
    }
}

impl ScoreEventType {
    pub fn get_score(&self) -> u32 {
        match self {
            ScoreEventType::NewHighscore(score) => *score,
            ScoreEventType::ReachedMilestone(score) => *score,
        }
    }
}
