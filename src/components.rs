use bevy::prelude::*;

// ───── Current Crate Imports ────────────────────────────────────────────── //

use crate::transition::TransitionRoute;

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

#[derive(Component)]
pub struct DustParticle {
    pub direction: Vec2,
    pub velocity: f32,
    pub timer: Timer,
}

#[derive(Component)]
pub struct Splash;

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
pub struct DarkenScreenEvent(pub TransitionRoute);
