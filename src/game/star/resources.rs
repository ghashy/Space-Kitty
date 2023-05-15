use bevy::prelude::*;

// ----- Constants ---------------------------------------------------------- //

const SPAWN_STAR_TIMER: f32 = 1.;

// ----- Body --------------------------------------------------------------- //

#[derive(Resource)]
pub struct StarSpawnTimer {
    pub timer: Timer,
}

impl Default for StarSpawnTimer {
    fn default() -> Self {
        StarSpawnTimer {
            timer: Timer::from_seconds(SPAWN_STAR_TIMER, TimerMode::Repeating),
        }
    }
}
