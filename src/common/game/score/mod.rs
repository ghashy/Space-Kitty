use bevy::prelude::*;

// ───── Current Crate Imports ────────────────────────────────────────────── //

use self::{resources::*, systems::*};
use crate::common::AppState;

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
            .add_systems(OnEnter(AppState::Game), insert_score)
            // Systems
            .add_systems(Update, update_chart_data)
            .add_systems(Update, update_highscores)
            // IMPORTANT: we should update score before we spawn
            // enemy on game progress, because spawning depends on
            // score value.
            .add_systems(First, update_score.run_if(in_state(AppState::Game)))
            // Exit State Systems
            .add_systems(OnExit(AppState::Game), remove_score)
            .add_systems(OnExit(AppState::GameOver), remove_highscore);
    }
}

// Events

#[derive(Event)]
pub struct ScoreUpdateEvent {
    pub name: Name,
    pub event_type: ScoreEventType,
}

pub enum ScoreEventType {
    ScoreDrop(u32),
    NewHighscore(u32),
    ReachedMilestone(u32),
}

impl ScoreUpdateEvent {
    pub fn new(name: Name, event_type: ScoreEventType) -> Self {
        ScoreUpdateEvent { name, event_type }
    }
}

impl ScoreEventType {
    pub fn get_score(&self) -> u32 {
        match self {
            ScoreEventType::NewHighscore(score) => *score,
            ScoreEventType::ReachedMilestone(score) => *score,
            ScoreEventType::ScoreDrop(score) => *score,
        }
    }
}
