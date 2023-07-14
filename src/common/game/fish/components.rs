use bevy::prelude::*;

#[derive(Component, Reflect)]
pub struct Fish {}

#[derive(Component)]
pub struct FishPack;

// Event
#[derive(Event)]
pub struct FishWasPickedEvent(pub Entity);
