use bevy::prelude::*;

// ───── Body ─────────────────────────────────────────────────────────────── //

#[derive(Component)]
pub struct FlyingMilk {
    pub direction: Vec2,
    pub rotation: f32,
    pub covered_distance: f32,
}
