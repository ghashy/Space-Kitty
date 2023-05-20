use bevy::prelude::*;

// ───── Body ─────────────────────────────────────────────────────────────── //

#[derive(Component)]
pub struct HUD;

#[derive(Component, Debug)]
pub enum HeartImage {
    Content(u8, Handle<Image>),
    Back(u8, Handle<Image>),
    Empty(u8, Handle<Image>),
}
