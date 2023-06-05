use bevy::prelude::*;

// ───── Body ─────────────────────────────────────────────────────────────── //

#[derive(Component)]
pub struct BackgroundStar {
    pub index: u8,
    pub timer: Timer,
}

#[derive(Component)]
pub struct BackgroundStars;

#[derive(Component)]
pub struct DarkScreenOverlap;

// Event
pub struct DarkenScreen;
