use bevy::{
    asset::{Asset, AssetLoader, LoadedAsset},
    prelude::*,
    reflect::TypeUuid,
};
use serde::Deserialize;

// ───── Body ─────────────────────────────────────────────────────────────── //

#[derive(Component)]
pub struct Enemy {
    pub direction: Vec2,
}

#[derive(Component)]
pub struct PatchOfLight;

// Event
pub struct EnemyIsArrivingEvent(pub String);
