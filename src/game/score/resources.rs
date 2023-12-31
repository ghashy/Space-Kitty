use std::collections::BTreeMap;

use bevy::{prelude::*, reflect::Map, utils::HashMap};

// ───── Body ─────────────────────────────────────────────────────────────── //

#[derive(Debug)]
pub enum ScoreError {
    NoScoreForEntity(Entity),
    TooLowScoreCount,
}

#[derive(Resource, Default)]
pub struct Score {
    pub data: HashMap<Entity, u32>,
}

impl Score {
    pub fn add_score_to(&mut self, who: &Entity, how_much: u32) -> u32 {
        let old_score = match self.data.get(who) {
            Some(score) => *score,
            None => 0,
        };
        let new_score = old_score + how_much;
        self.data.insert(*who, new_score);
        new_score
    }

    /// Drops score from entity and returns new score count, how much is in
    /// range from 0 to 1 (percentage)
    pub fn drop_score(
        &mut self,
        who: Entity,
        percentage: f32,
    ) -> Result<u32, ScoreError> {
        if let Some(current) = self.data.get_mut(&who) {
            if *current > 5 {
                let drop_value = (*current as f32 * percentage) as u32;
                *current -= drop_value;
                return Ok(drop_value);
            } else {
                return Err(ScoreError::TooLowScoreCount);
            }
        }
        Err(ScoreError::NoScoreForEntity(who))
    }

    pub fn get_score(&self, for_who: &Entity) -> Result<u32, ScoreError> {
        match self.data.get(for_who) {
            Some(score) => Ok(*score),
            None => Err(ScoreError::NoScoreForEntity(*for_who)),
        }
    }
}

// For store on the disk
#[derive(Resource, Debug)]
pub struct HighScores {
    pub scores: HashMap<Name, (Handle<Image>, u32)>,
}

impl Default for HighScores {
    fn default() -> Self {
        HighScores {
            scores: HashMap::new(),
        }
    }
}

impl HighScores {
    pub fn sorted(
        &self,
    ) -> std::iter::Rev<std::vec::IntoIter<(u32, Name, Handle<Image>)>> {
        let mut vec: Vec<(u32, Name, Handle<Image>)> = self
            .scores
            .iter()
            .map(|(name, (image, score))| (*score, name.clone(), image.clone()))
            .collect();
        // vec.sort_by(|&(score1, _, _), &(score2, _, _)| score2.cmp(&score1));
        vec.sort_by_key(|&(score, _, _)| score);
        vec.into_iter().rev()
    }
}

#[derive(Debug)]
pub struct ScoreLine {
    pub name: Name,
    pub entity: Entity,
    pub image: Handle<Image>,
    pub score: u32,
}

// For in-game top chart
#[derive(Resource, Default)]
pub struct Chart {
    pub lines: Vec<ScoreLine>,
}

impl Chart {
    pub fn get_pos(&self, entity: Entity) -> Option<usize> {
        self.lines.iter().position(|line| line.entity == entity)
    }

    pub fn get_line(&self, pos: usize) -> Option<&ScoreLine> {
        self.lines.get(pos)
    }
}
