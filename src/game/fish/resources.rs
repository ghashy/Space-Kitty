use bevy::prelude::*;

// ───── Constants ────────────────────────────────────────────────────────── //

const SPAWN_STAR_TIMER: f32 = 0.3;

// ───── Body ─────────────────────────────────────────────────────────────── //

#[derive(Resource)]
pub struct FishSpawnTimer {
    pub timer: Timer,
}

impl Default for FishSpawnTimer {
    fn default() -> Self {
        FishSpawnTimer {
            timer: Timer::from_seconds(SPAWN_STAR_TIMER, TimerMode::Repeating),
        }
    }
}
