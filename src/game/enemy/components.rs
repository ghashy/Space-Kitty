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
    pub phrase_timer: Timer,
}

#[derive(Component)]
pub struct PatchOfLight;

#[derive(Component)]
pub struct MessageBox(pub Timer, pub Option<Timer>);

#[derive(Component)]
pub struct BoyAnimation(pub Timer, pub u8);
