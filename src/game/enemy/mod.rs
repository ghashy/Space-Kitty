use bevy::prelude::*;

// ───── Current Crate Imports ────────────────────────────────────────────── //

use self::{assets::DogData, resources::DogResource, systems::*};
use super::SimulationState;
use crate::AppState;

// ───── Submodules ───────────────────────────────────────────────────────── //

pub mod assets;
pub mod components;
pub mod resources;
pub mod systems;

// ───── Constants ────────────────────────────────────────────────────────── //

const ENEMY_SPEED: f32 = 10000.;

// ───── Body ─────────────────────────────────────────────────────────────── //

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app
            // Assets
            .add_asset::<DogData>()
            // Events
            .add_event::<EnemyIsArrivingEvent>()
            .add_event::<MessageBoxRequest>()
            // Enter State Systems
            .add_system(load_resources.in_schedule(OnEnter(AppState::Game)))
            // Systems
            .add_systems(
                (
                    update_enemy_direction,
                    enemy_movement,
                    enemy_chatting,
                    spawn_enemy_on_game_progress,
                    rotate_patch_of_light,
                    update_message_box,
                    system_add_collider_to_enemy,
                    spawn_message_box,
                )
                    .in_set(OnUpdate(AppState::Game))
                    .in_set(OnUpdate(SimulationState::Running)),
            )
            // Exit State Systems
            .add_system(despawn_enemies.in_schedule(OnExit(AppState::Game)));
    }
}

// Events
pub struct EnemyIsArrivingEvent(pub String);

pub struct MessageBoxRequest(Entity, String);
