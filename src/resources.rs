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
