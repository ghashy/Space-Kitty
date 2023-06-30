use bevy::prelude::*;

// ───── Current Crate Imports ────────────────────────────────────────────── //

use super::assets::DogData;

// ───── Body ─────────────────────────────────────────────────────────────── //

#[derive(Resource, Default)]
pub struct DogResource {
    pub json_data: Handle<DogData>,
    pub images: Vec<(String, Handle<Image>)>,
    pub avatars: Vec<Handle<Image>>,
}
