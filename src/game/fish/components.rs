use bevy::prelude::*;

#[derive(Component, Reflect)]
pub struct Fish {}

#[derive(Component)]
pub struct FishPack;

// Event
pub struct FishWasPickedEvent(pub String);
