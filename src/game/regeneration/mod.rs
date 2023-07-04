use bevy::prelude::*;

// ───── Current Crate Imports ────────────────────────────────────────────── //

use crate::AppState;

use self::resources::FlyingMilkResource;
use self::systems::{
    check_collision, cup_of_milk_movement, despawn_milk, spawn_milk_cup,
};

// ───── Submodules ───────────────────────────────────────────────────────── //

// Top-level modules
mod components;
mod resources;
mod systems;

// ───── Constants ────────────────────────────────────────────────────────── //

const MILK_SPEED: f32 = 250.;
const MAX_SPAWN_TIME: f32 = 50.;

// ───── Body ─────────────────────────────────────────────────────────────── //

pub struct RegenerationPlugin;

impl Plugin for RegenerationPlugin {
    fn build(&self, app: &mut App) {
        app
            // Resources
            .init_resource::<FlyingMilkResource>()
            // Events
            .add_event::<RegeneratePlayerEvent>()
            // Systems
            .add_systems(
                (
                    spawn_milk_cup,
                    cup_of_milk_movement,
                    check_collision,
                    despawn_milk,
                )
                    .in_set(OnUpdate(AppState::Game)),
            );
    }
}

// Events

pub struct RegeneratePlayerEvent {
    pub new_health: u8,
}
