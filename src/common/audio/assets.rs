use bevy::reflect::TypeUuid;
use kira::sound::static_sound::StaticSoundData;

// ───── Body ─────────────────────────────────────────────────────────────── //

#[derive(TypeUuid)]
#[uuid = "c1583df3-885a-4d8d-ad38-530118f6c004"]
pub struct AudioSource {
    pub static_sound_data: StaticSoundData,
}

impl AudioSource {
    pub fn get(&self) -> StaticSoundData {
        self.static_sound_data.clone()
    }
}
