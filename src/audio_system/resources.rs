use bevy::prelude::*;
use bevy_kira_audio::AudioSource;

#[derive(Resource)]
pub struct SamplePack {
    pub imp_light_0: Handle<AudioSource>,
    pub imp_light_1: Handle<AudioSource>,
    pub imp_light_2: Handle<AudioSource>,
    pub imp_light_3: Handle<AudioSource>,
    pub imp_light_4: Handle<AudioSource>,
    pub imp_med_0: Handle<AudioSource>,
    pub imp_med_1: Handle<AudioSource>,
    pub exp: Handle<AudioSource>,
    pub pick_star: Handle<AudioSource>,
}
