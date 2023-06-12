use bevy::prelude::*;

// ───── Current Crate Imports ────────────────────────────────────────────── //

use super::resources::{HighScores, Score};
use crate::{
    events::GameOver,
    game::{
        enemy::components::Enemy, fish::components::FishWasPickedEvent,
        player::components::Player,
    },
};

// ───── Body ─────────────────────────────────────────────────────────────── //

pub fn insert_score(mut commands: Commands) {
    commands.insert_resource(Score::default());
}

pub fn remove_score(mut commands: Commands) {
    commands.remove_resource::<Score>();
}

// TODO: Implement highscore system
pub fn update_highscores(
    mut _game_over_event_reader: EventReader<GameOver>,
    mut _high_scores: ResMut<HighScores>,
) {
    // for event in game_over_event_reader.iter() {
    //     high_scores
    //         .scores
    //         .push(("Player".to_string(), event.final_score));
    // }
}

pub fn high_scores_updated(high_scores: ResMut<HighScores>) {
    if high_scores.is_changed() {
        println!("High Scores: {:?}", high_scores);
    }
}

pub fn update_score(
    mut picked_event: EventReader<FishWasPickedEvent>,
    entity_query: Query<
        (&Name, &Handle<Image>),
        Or<(With<Player>, With<Enemy>)>,
    >,
    mut score: ResMut<Score>,
) {
    for event in picked_event.iter() {
        for (name, image) in entity_query.iter() {
            if name.as_str() == event.0 {
                score.add_score_to(&event.0, image.clone());
            }
        }
    }
}
