use bevy::prelude::*;

// ───── Current Crate Imports ────────────────────────────────────────────── //

use self::{
    assets::AudioSource,
    audio_loader::AudioLoader,
    resources::{KiraManager, SoundHandleResource},
};

// ───── Submodules ───────────────────────────────────────────────────────── //

pub mod assets;
pub mod audio_loader;
pub mod resources;

// ───── Body ─────────────────────────────────────────────────────────────── //

/// This audio plugin requires all samples to be loaded before they will be
/// played!
pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_asset::<AudioSource>()
            .init_asset_loader::<AudioLoader>()
            .init_non_send_resource::<KiraManager>()
            .init_resource::<SoundHandleResource>();
    }
}
