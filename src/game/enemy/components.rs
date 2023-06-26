use bevy::prelude::*;

// ───── Body ─────────────────────────────────────────────────────────────── //

#[derive(Clone, Copy)]
pub enum DogType {
    Ordinary,
    BigBoy,
    Harry,
}

#[derive(Component)]
pub struct Enemy {
    pub dog_type: DogType,
    pub direction: Vec2,
    pub has_collider: bool,
    pub scale: f32,
}

#[derive(Component)]
pub struct PatchOfLight;

#[derive(Component)]
pub struct MessageBox(pub Timer, pub Option<Timer>);
