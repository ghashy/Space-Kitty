use bevy::prelude::*;

// ───── Body ─────────────────────────────────────────────────────────────── //

#[derive(Resource, Default)]
pub struct FlyingMilkResource {
    pub timer: Option<Timer>,
}
