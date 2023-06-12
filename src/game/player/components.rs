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
