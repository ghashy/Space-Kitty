use bevy::prelude::*;

// ───── Current Crate Imports ────────────────────────────────────────────── //

use crate::common::AppState;

use self::resources::FlyingMilkResource;
use self::systems::{
    check_collision, cup_of_milk_movement, despawn_milk_on_exit_state,
    despawn_milk_out_of_screen, spawn_milk_cup,
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
            .add_event::<MilkEscapedEvent>()
            // Systems
            .add_systems(
                Update,
                (
                    spawn_milk_cup,
                    cup_of_milk_movement,
                    check_collision,
                    despawn_milk_out_of_screen,
                )
                    .run_if(in_state(AppState::Game)),
            )
            // Exit State Systems
            .add_systems(OnExit(AppState::Game), despawn_milk_on_exit_state);
    }
}

// Events

#[derive(Event)]
pub struct RegeneratePlayerEvent {
    pub new_health: u8,
}

#[derive(Event)]
pub struct MilkEscapedEvent;
