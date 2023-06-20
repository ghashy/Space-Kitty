use bevy::prelude::*;

// ───── Current Crate Imports ────────────────────────────────────────────── //

use super::assets::DogNames;

// ───── Body ─────────────────────────────────────────────────────────────── //

#[derive(Resource, Default)]
pub struct DogResource {
    pub json_data: Handle<DogNames>,
    pub images: Vec<(String, Handle<Image>)>,
}
