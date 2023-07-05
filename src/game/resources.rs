use bevy::prelude::*;

// ───── Body ─────────────────────────────────────────────────────────────── //

#[derive(Resource, Default)]
pub struct GameData {
    pub sheet_was_shown: bool,
}
