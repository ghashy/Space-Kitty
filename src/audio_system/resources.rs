use bevy::prelude::*;

#[derive(Resource)]
pub struct SamplePack {
    pub pluck1: Handle<AudioSource>,
    pub pluck2: Handle<AudioSource>,
    pub exp: Handle<AudioSource>,
    pub pick_star: Handle<AudioSource>,
}
