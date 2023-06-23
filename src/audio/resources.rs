use bevy::prelude::*;
use kira::manager::{
    backend::DefaultBackend, AudioManager, AudioManagerSettings,
};
use kira::sound::static_sound::StaticSoundHandle;
use std::ops::Deref;
use std::ops::DerefMut;

// ───── Current Crate Imports ────────────────────────────────────────────── //

use super::assets::AudioSource;

// ───── Body ─────────────────────────────────────────────────────────────── //

#[derive(Resource, Default)]
pub struct SoundHandleResource {
    pub title_theme: Option<StaticSoundHandle>,
    pub main_theme: Option<StaticSoundHandle>,
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
}

impl Default for KiraManager {
    fn default() -> Self {
        KiraManager {
            manager: AudioManager::<DefaultBackend>::new(
                AudioManagerSettings::default(),
            )
            .unwrap(),
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
