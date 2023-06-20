use bevy::asset::{AssetLoader, LoadedAsset};

// ───── Current Crate Imports ────────────────────────────────────────────── //

use crate::game::enemy::assets::DogNames;

// ───── Body ─────────────────────────────────────────────────────────────── //

#[derive(Default)]
pub struct JsonAssetLoader;

impl AssetLoader for JsonAssetLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut bevy::asset::LoadContext,
    ) -> bevy::utils::BoxedFuture<'a, Result<(), bevy::asset::Error>> {
        Box::pin(async move {
            let custom_asset = serde_json::from_slice::<DogNames>(bytes)?;
            load_context.set_default_asset(LoadedAsset::new(custom_asset));
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["json"]
    }
}
