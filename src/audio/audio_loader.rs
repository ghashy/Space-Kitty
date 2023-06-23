use bevy::{
    asset::{AssetLoader, LoadContext, LoadedAsset},
    utils::BoxedFuture,
};
use kira::sound::static_sound::{StaticSoundData, StaticSoundSettings};
use std::io::Cursor;

// ───── Current Crate Imports ────────────────────────────────────────────── //

use super::assets::AudioSource;

// ───── Body ─────────────────────────────────────────────────────────────── //

#[derive(Default)]
pub struct AudioLoader;

impl AssetLoader for AudioLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<(), bevy::asset::Error>> {
        Box::pin(async move {
            let mut sound_bytes = vec![];
            for byte in bytes {
                sound_bytes.push(*byte);
            }
            let static_sound_data = StaticSoundData::from_cursor(
                Cursor::new(sound_bytes),
                StaticSoundSettings::default(),
            )?;
            load_context.set_default_asset(LoadedAsset::new(AudioSource {
                static_sound_data,
            }));
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["ogg", "wav"]
    }
}
