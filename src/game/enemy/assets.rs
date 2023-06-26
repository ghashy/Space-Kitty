use bevy::reflect::TypeUuid;
use serde::Deserialize;

// ───── Body ─────────────────────────────────────────────────────────────── //

#[derive(Clone, Debug, Deserialize, TypeUuid)]
#[uuid = "a51952db-28e3-4bac-ba92-0d3c90921985"]
pub struct DogData {
    pub first_names: Vec<String>,
    pub last_names: Vec<String>,
    pub nicknames: Vec<String>,
    pub hellos: Vec<String>,
    pub wildly_rotations: Vec<String>,
    pub fish_picking: Vec<String>,
}
