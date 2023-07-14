use std::time::Duration;

use bevy::prelude::*;

// ───── Body ─────────────────────────────────────────────────────────────── //

#[derive(Resource)]
pub struct CometTimer(pub Timer);

impl Default for CometTimer {
    fn default() -> Self {
        CometTimer(Timer::new(Duration::from_secs(3), TimerMode::Repeating))
    }
}

#[derive(Resource)]
pub struct TextureStorage {
    pub(super) komet_blue: Handle<Image>,
    pub(super) komet_purple: Handle<Image>,
    pub(super) komet_red: Handle<Image>,
    pub(super) glowing_star: Handle<Image>,
    pub smoke: Handle<Image>,
    pub note1: Handle<Image>,
    pub note2: Handle<Image>,
    pub note3: Handle<Image>,
    pub note4: Handle<Image>,
    pub note5: Handle<Image>,
    pub note6: Handle<Image>,
    pub note7: Handle<Image>,
    pub note8: Handle<Image>,
}

impl FromWorld for TextureStorage {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        TextureStorage {
            komet_blue: asset_server.load("sprites/Komet Blue.png"),
            komet_purple: asset_server.load("sprites/Komet Purple.png"),
            komet_red: asset_server.load("sprites/Komet Red.png"),
            smoke: asset_server.load("sprites/Smoke.png"),
            glowing_star: asset_server.load("sprites/Star glowing.png"),
            note1: asset_server.load("sprites/Notes/Dotted half note.png"),
            note2: asset_server.load("sprites/Notes/Eighth note.png"),
            note3: asset_server.load("sprites/Notes/Eighth notes down.png"),
            note4: asset_server.load("sprites/Notes/Eighth notes up.png"),
            note5: asset_server.load("sprites/Notes/Half note.png"),
            note6: asset_server.load("sprites/Notes/Quarter note down.png"),
            note7: asset_server.load("sprites/Notes/Quarter note up.png"),
            note8: asset_server.load("sprites/Notes/Sixteenth notes.png"),
        }
    }
}

#[derive(Resource)]
pub struct DustTimer(pub Timer);

impl Default for DustTimer {
    fn default() -> Self {
        DustTimer(Timer::from_seconds(0.05, TimerMode::Repeating))
    }
}
