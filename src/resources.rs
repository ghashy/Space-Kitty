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
        }
    }
}
