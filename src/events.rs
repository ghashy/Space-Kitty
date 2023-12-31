use bevy::prelude::*;

// ───── Events ───────────────────────────────────────────────────────────── //

pub struct GameOver;

pub struct PlayerHit {
    pub remaining_health: u8,
    pub position: Vec3,
    pub hit_normal: Vec2,
    pub drop_count: u32,
}
