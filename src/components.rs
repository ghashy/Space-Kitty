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

#[derive(Component)]
pub struct Comet {
    pub speed_modifier: f32,
    pub resolution: Vec2,
}

impl Comet {
    pub fn new(speed_modifier: f32, resolution: Vec2) -> Comet {
        Comet {
            speed_modifier,
            resolution,
        }
    }
}

#[derive(Component)]
pub struct Comets;

// Event
pub struct DarkenScreenEvent;
