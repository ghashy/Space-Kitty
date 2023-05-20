use bevy::prelude::*;

// ───── Current Crate Imports ────────────────────────────────────────────── //

use super::resources::{HighScores, Score};
use crate::events::GameOver;

// ───── Body ─────────────────────────────────────────────────────────────── //

pub fn insert_score(mut commands: Commands) {
    commands.insert_resource(Score::default());
}

pub fn remove_score(mut commands: Commands) {
    commands.remove_resource::<Score>();
}

pub fn update_score(score: Res<Score>) {
    if score.is_changed() {
        println!("Score: {}", score.value.to_string());
    }
}

pub fn update_highscores(
    mut game_over_event_reader: EventReader<GameOver>,
    mut high_scores: ResMut<HighScores>,
) {
    for event in game_over_event_reader.iter() {
        high_scores
            .scores
            .push(("Player".to_string(), event.final_score));
    }
}

pub fn high_scores_updated(high_scores: ResMut<HighScores>) {
    if high_scores.is_changed() {
        println!("High Scores: {:?}", high_scores);
    }
}
