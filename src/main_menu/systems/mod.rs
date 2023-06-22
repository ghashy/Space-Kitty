use bevy::prelude::*;
use bevy_kira_audio::{
    AudioChannel, AudioControl, AudioEasing, AudioTween, MainTrack, *,
};

use crate::audio_system::resources::SamplePack;

// ───── Submodules ───────────────────────────────────────────────────────── //

pub mod interactions;
pub mod layout;

// ───── Body ─────────────────────────────────────────────────────────────── //

pub fn play_title_theme(
    audio: Res<AudioChannel<MainTrack>>,
    sample_pack: Res<SamplePack>,
) {
    let mut a = audio.play(sample_pack.main_theme.clone_weak()).handle();
}
