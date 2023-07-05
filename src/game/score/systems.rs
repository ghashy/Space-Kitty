use bevy::prelude::*;
use rand::Rng;

// ───── Current Crate Imports ────────────────────────────────────────────── //

use super::{
    resources::{Chart, HighScores, Score, ScoreLine},
    ScoreUpdateEvent,
};
use crate::game::{
    enemy::{components::Enemy, resources::DogResource},
    fish::components::FishWasPickedEvent,
    gui::components::Avatar,
    player::components::Player,
};

// ───── Body ─────────────────────────────────────────────────────────────── //

pub fn insert_score(mut commands: Commands) {
    commands.insert_resource(Score::default());
}

pub fn remove_score(mut commands: Commands) {
    commands.remove_resource::<Score>();
}

pub fn update_chart_data(
    entities_query: Query<
        (Entity, &Avatar, &Name),
        Or<(With<Player>, With<Enemy>)>,
    >,
    scores: Option<Res<Score>>,
    mut chart: ResMut<Chart>,
) {
    if let Some(scores) = scores {
        if scores.is_changed() {
            let mut scores = scores.data.iter().collect::<Vec<_>>();
            scores.sort_by(|a, b| b.1.cmp(&a.1));
            chart.lines = scores
                .iter()
                .map(|&(&entity, &score)| {
                    entities_query.iter().find_map(|(e, avatar, name)| {
                        if e == entity {
                            Some(ScoreLine {
                                image: avatar.0.clone(),
                                name: name.clone(),
                                entity,
                                score,
                            })
                        } else {
                            None
                        }
                    })
                })
                .flatten()
                .collect();
        }
    }
}

pub fn update_highscores(
    entities_query_sprites: Query<
        (Entity, &Handle<Image>, &Name),
        Or<(With<Player>, With<Enemy>)>,
    >,
    entities_query_atlases: Query<
        (Entity, &Handle<TextureAtlas>, &Name),
        With<Enemy>,
    >,
    scores: Option<Res<Score>>,
    mut highscores: ResMut<HighScores>,
    dogs_resource: Option<Res<DogResource>>,
) {
    if let Some(scores) = scores {
        if scores.is_changed() {
            for (&entity, &score) in scores.data.iter() {
                let iterator = entities_query_sprites.iter().chain(
                    entities_query_atlases.iter().map(|(e, _, name)| {
                        (
                            e,
                            &dogs_resource
                                .as_ref()
                                .unwrap()
                                .dogs
                                .iter()
                                .find(|&dog| {
                                    dog.texture_identifier == "FaceBigBoy"
                                })
                                .unwrap()
                                .texture,
                            name,
                        )
                    }),
                );

                for (e, image, name) in iterator {
                         if e == entity {
                        highscores
                            .scores
                            .insert(name.clone(), (image.clone(), score));
                    }
                }
            }
        }
    }
}

pub fn update_score(
    mut picked_event: EventReader<FishWasPickedEvent>,
    mut score_update_event: EventWriter<ScoreUpdateEvent>,
    entity_query: Query<(&Name, Entity), Or<(With<Player>, With<Enemy>)>>,
    mut score: ResMut<Score>,
) {
    for event in picked_event.iter() {
        for (name, entity) in entity_query.iter() {
            if entity == event.0 {
                let loot_count = if name.to_string() == "Kitty" {
                    1
                } else {
                    rand::thread_rng().gen_range(1..10)
                };

                let new_score = score.add_score_to(&entity, loot_count);

                if new_score % 7 == 0 || new_score == 1 {
                    score_update_event.send(ScoreUpdateEvent::new(
                        name.clone(),
                        super::ScoreEventType::ReachedMilestone(new_score),
                    ))
                }
            }
        }
    }
}
