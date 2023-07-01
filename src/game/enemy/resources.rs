use bevy::prelude::*;

// ───── Current Crate Imports ────────────────────────────────────────────── //

use super::assets::DogData;

// ───── Body ─────────────────────────────────────────────────────────────── //

pub struct OneDog {
    pub texture_identifier: String,
    pub texture: Handle<Image>,
    pub avatar: Handle<Image>,
}

#[derive(Resource, Default)]
pub struct DogResource {
    pub json_data: Handle<DogData>,
    pub dogs: Vec<OneDog>,
}
