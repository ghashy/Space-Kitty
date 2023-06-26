use bevy::prelude::*;

// ───── Current Crate Imports ────────────────────────────────────────────── //

use super::{
    resources::{Chart, HighScores, Score},
    ScoreUpdateEvent,
};
use crate::game::{
    enemy::components::Enemy, fish::components::FishWasPickedEvent,
    player::components::Player,
};

// ───── Body ─────────────────────────────────────────────────────────────── //

pub fn insert_score(mut commands: Commands) {
    commands.insert_resource(Score::default());
}

pub fn remove_score(mut commands: Commands) {
    commands.remove_resource::<Score>();
}

pub fn sort_highscores(scores: Option<Res<Score>>, mut chart: ResMut<Chart>) {
    if let Some(scores) = scores {
        if scores.is_changed() {
            let mut vec = scores.data.iter().collect::<Vec<_>>();
            vec.sort_by(|a, b| a.1 .1.cmp(&b.1 .1));

            chart.top1 =
                vec.get(0).map(|&v| (v.0.clone(), v.1 .0.clone(), v.1 .1));
            chart.top2 =
                vec.get(1).map(|&v| (v.0.clone(), v.1 .0.clone(), v.1 .1));
            chart.top3 =
                vec.get(2).map(|&v| (v.0.clone(), v.1 .0.clone(), v.1 .1));
        }
    }
}

pub fn update_score(
    mut picked_event: EventReader<FishWasPickedEvent>,
    mut score_update_event: EventWriter<ScoreUpdateEvent>,
    entity_query: Query<
        (&Name, &Handle<Image>),
        Or<(With<Player>, With<Enemy>)>,
    >,
    mut score: ResMut<Score>,
) {
    for event in picked_event.iter() {
        for (name, image) in entity_query.iter() {
            if name.as_str() == event.0 {
                let new_score = score.add_score_to(&event.0, image.clone());

                if new_score % 7 == 0 || new_score == 1 {
                    score_update_event.send(ScoreUpdateEvent::new(
                        name.into(),
                        super::ScoreEventType::ReachedMilestone(new_score),
                    ))
                }
            }
        }
    }
}
