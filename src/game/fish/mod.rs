use bevy::prelude::*;

// ───── Current Crate Imports ────────────────────────────────────────────── //

use self::{
    components::{Fish, FishWasPickedEvent},
    resources::*,
    systems::*,
};
use super::SimulationState;
use crate::AppState;

// ───── Submodules ───────────────────────────────────────────────────────── //

pub mod components;
pub mod resources;
pub mod systems;

// ───── Constants ────────────────────────────────────────────────────────── //

const NUMBER_OF_FISH: usize = 10;
pub const FISH_SIZE: Vec2 = Vec2::new(168. / 5.9, 130. / 5.9);

// ───── Body ─────────────────────────────────────────────────────────────── //

pub struct FishPlugin;

impl Plugin for FishPlugin {
    fn build(&self, app: &mut App) {
        app
            // Register types
            .register_type::<Fish>()
            // Resources
            .init_resource::<FishSpawnTimer>()
            // Events
            .add_event::<FishWasPickedEvent>()
            // Enter State Systems
            .add_system(spawn_fish.in_schedule(OnEnter(AppState::Game)))
            // Systems
            .add_systems(
                (tick_fish_spawn_timer, spawn_fish_over_time, check_collision)
                    .in_set(OnUpdate(AppState::Game))
                    .in_set(OnUpdate(SimulationState::Running)),
            )
            // Exit State Systems
            .add_system(despawn_fish.in_schedule(OnExit(AppState::Game)));
    }
}
