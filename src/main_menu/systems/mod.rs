use std::time::Duration;

use bevy::prelude::*;

use crate::audio::{
    assets::AudioSource,
    resources::{KiraManager, SamplePack, SoundHandleResource},
};

// ───── Submodules ───────────────────────────────────────────────────────── //

pub mod interactions;
pub mod layout;

// ───── Body ─────────────────────────────────────────────────────────────── //

pub fn play_title_theme(
    mut kira_manager: NonSendMut<KiraManager>,
    audio_assets: Res<Assets<AudioSource>>,
    sample_pack: Res<SamplePack>,
    mut sound_handle: ResMut<SoundHandleResource>,
) {
    let mut handle = kira_manager
        .play(audio_assets.get(&sample_pack.title_theme).unwrap().get())
        .unwrap();
    handle.set_loop_region(..).unwrap();
    sound_handle.title_theme = Some(handle);
}

pub fn stop_title_theme(mut sound_handle: ResMut<SoundHandleResource>) {
    if let Some(ref mut handle) = sound_handle.title_theme {
        handle
            .stop(kira::tween::Tween {
                duration: Duration::from_millis(200),
                ..default()
            })
            .unwrap();
    }
}
