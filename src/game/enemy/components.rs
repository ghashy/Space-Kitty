use bevy::prelude::*;

// ───── Body ─────────────────────────────────────────────────────────────── //

#[derive(Component)]
pub struct Enemy {
    pub direction: Vec2,
    pub has_collider: bool,
    pub scale: f32,
}

#[derive(Component)]
pub struct PatchOfLight;

#[derive(Component)]
pub struct MessageBox;
