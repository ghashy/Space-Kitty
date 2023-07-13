use bevy::prelude::*;

#[derive(Component)]
pub struct Player {
    pub health: u8,
}

#[derive(Component)]
pub struct PlayerInvulnerableTimer(pub Timer);

#[derive(Component)]
pub struct RocketEngineParticles;

#[derive(Component)]
pub struct RocketEngineSprite;

#[derive(Component)]
pub struct DropFishParticle {
    pub direction: Vec2,
    pub velocity: f32,
    pub timer: Timer,
}

#[derive(Component)]
pub struct SmokeParticle {
    pub direction: Vec2,
    pub velocity: f32,
    pub timer: Timer,
}
