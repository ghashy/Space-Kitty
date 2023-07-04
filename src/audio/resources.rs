use bevy::prelude::*;
use kira::clock::ClockHandle;
use kira::manager::{
    backend::DefaultBackend, AudioManager, AudioManagerSettings,
};
use kira::sound::static_sound::StaticSoundHandle;
use kira::track::effect::compressor::CompressorBuilder;
use kira::track::{TrackBuilder, TrackHandle};
use std::ops::Deref;
use std::ops::DerefMut;
use std::time::Duration;

// ───── Current Crate Imports ────────────────────────────────────────────── //

use super::assets::AudioSource;

// ───── Body ─────────────────────────────────────────────────────────────── //

#[derive(Resource, Default)]
pub struct SoundHandleResource {
    pub title_theme: Option<StaticSoundHandle>,
    pub main_theme: Option<StaticSoundHandle>,
    pub main_theme_clock: Option<ClockHandle>,
}

#[derive(Resource)]
pub struct SamplePack {
    pub pick_fish1: Handle<AudioSource>,
    pub pick_fish2: Handle<AudioSource>,
    pub pick_fish3: Handle<AudioSource>,
    pub pick_fish4: Handle<AudioSource>,
    pub pick_fish5: Handle<AudioSource>,
    pub pick_fish6: Handle<AudioSource>,
    pub pick_fish7: Handle<AudioSource>,
    pub pick_fish8: Handle<AudioSource>,
    pub pick_fish9: Handle<AudioSource>,
    pub pick_fish10: Handle<AudioSource>,
    pub pick_fish11: Handle<AudioSource>,
    pub pick_fish12: Handle<AudioSource>,
    pub pick_fish13: Handle<AudioSource>,
    pub pick_fish14: Handle<AudioSource>,
    pub pick_fish15: Handle<AudioSource>,
    pub pick_fish16: Handle<AudioSource>,
    pub pick_fish17: Handle<AudioSource>,
    pub pick_fish18: Handle<AudioSource>,
    pub pick_fish19: Handle<AudioSource>,
    pub pick_fish20: Handle<AudioSource>,
    pub pick_fish21: Handle<AudioSource>,
    pub bark1: Handle<AudioSource>,
    pub bark2: Handle<AudioSource>,
    pub bark3: Handle<AudioSource>,
    pub bark4: Handle<AudioSource>,
    pub bark5: Handle<AudioSource>,
    pub bark6: Handle<AudioSource>,
    pub bark7: Handle<AudioSource>,
    pub bark8: Handle<AudioSource>,
    pub bark9: Handle<AudioSource>,
    pub bark10: Handle<AudioSource>,
    pub bark11: Handle<AudioSource>,
    pub bark12: Handle<AudioSource>,
    pub meow1: Handle<AudioSource>,
    pub meow2: Handle<AudioSource>,
    pub meow3: Handle<AudioSource>,
    pub meow4: Handle<AudioSource>,
    pub meow5: Handle<AudioSource>,
    pub meow6: Handle<AudioSource>,
    pub meow7: Handle<AudioSource>,
    pub meowroar: Handle<AudioSource>,
    pub wall_collision: Handle<AudioSource>,
    pub title_theme: Handle<AudioSource>,
    pub main_theme: Handle<AudioSource>,
    pub alarm: Handle<AudioSource>,
    pub engine: Handle<AudioSource>,
}

pub struct SamplePackIterator<'a> {
    sample_pack: &'a SamplePack,
    current: usize,
}

impl SamplePack {
    pub fn iter(&self) -> SamplePackIterator {
        SamplePackIterator {
            current: 0,
            sample_pack: self,
        }
    }
}

impl<'a> Iterator for SamplePackIterator<'a> {
    type Item = &'a Handle<AudioSource>;
    fn next(&mut self) -> Option<Self::Item> {
        match self.current {
            0 => {
                self.current += 1;
                Some(&self.sample_pack.pick_fish1)
            }
            1 => {
                self.current += 1;
                Some(&self.sample_pack.title_theme)
            }
            2 => {
                self.current += 1;
                Some(&self.sample_pack.main_theme)
            }
            3 => {
                self.current += 1;
                Some(&self.sample_pack.alarm)
            }
            4 => {
                self.current += 1;
                Some(&self.sample_pack.engine)
            }
            5 => {
                self.current = 0;
                None
            }
            _ => unreachable!(),
        }
    }
}

// Non send resource
pub struct KiraManager {
    manager: AudioManager,
    master_track: TrackHandle,
}

impl KiraManager {
    pub fn get_master(&self) -> &TrackHandle {
        &self.master_track
    }
}

impl Default for KiraManager {
    fn default() -> Self {
        let mut manager = AudioManager::<DefaultBackend>::new(
            AudioManagerSettings::default(),
        )
        .unwrap();
        manager
            .main_track()
            .set_volume(0.8, kira::tween::Tween::default())
            .unwrap();
        let master_track = manager
            .add_sub_track(TrackBuilder::new().with_effect(
                // CompressorBuilder::new()
                //     .ratio(10.)
                //     .threshold(-2.)
                //     .attack_duration(Duration::from_millis(1))
                //     .release_duration(Duration::from_millis(100)),
                kira::track::effect::reverb::ReverbBuilder::new(),
            ))
            .unwrap();
        KiraManager {
            manager,
            master_track,
        }
    }
}

impl Deref for KiraManager {
    type Target = AudioManager;

    fn deref(&self) -> &Self::Target {
        &self.manager
    }
}

impl DerefMut for KiraManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.manager
    }
}
