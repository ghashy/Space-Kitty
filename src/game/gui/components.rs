use bevy::prelude::*;

// ----- Body --------------------------------------------------------------- //

#[derive(Component)]
pub struct HUD;

#[derive(Component, Debug)]
pub enum HeartImage {
    Content(u8),
    Back(u8),
    Empty(u8),
}
