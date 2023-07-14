use bevy::prelude::*;

// ───── Events ───────────────────────────────────────────────────────────── //

#[derive(Event)]
pub struct GameOver;

#[derive(Event)]
pub struct PlayerHit {
    pub remaining_health: u8,
    pub position: Vec3,
    pub hit_normal: Vec2,
    pub drop_count: u32,
}
