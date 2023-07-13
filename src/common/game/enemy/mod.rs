use bevy::prelude::*;

// ───── Current Crate Imports ────────────────────────────────────────────── //

use self::{assets::DogData, resources::DogResource, systems::*};
use super::SimulationState;
use crate::common::AppState;

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
            .add_event::<DoggyTheme>()
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
                    emit_notes,
                    poll_and_despawn_notes,
                )
                    .in_set(OnUpdate(AppState::Game))
                    .in_set(OnUpdate(SimulationState::Running)),
            )
            .add_system(animate_big_boy.in_set(OnUpdate(AppState::Game)))
            // Exit State Systems
            .add_systems(
                (despawn_enemies, despawn_notes_on_exit)
                    .in_schedule(OnExit(AppState::Game)),
            );
    }
}

// Events
pub struct EnemyIsArrivingEvent(pub String);

pub struct MessageBoxRequest(Entity, String);

pub struct DoggyTheme;
