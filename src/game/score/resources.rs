use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct Score {
    pub value: u32,
    pub last_value: u32,
}

#[derive(Resource, Debug)]
pub struct HighScores {
    pub scores: Vec<(String, u32)>,
}

impl Default for HighScores {
    fn default() -> Self {
        HighScores { scores: Vec::new() }
    }
}
