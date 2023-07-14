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
            .add_systems(OnEnter(AppState::Game), load_resources)
            // Systems
            .add_systems(
                Update,
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
                    .run_if(in_state(AppState::Game))
                    .run_if(in_state(SimulationState::Running)),
            )
            .add_systems(
                Update,
                animate_big_boy.run_if(in_state(AppState::Game)),
            )
            // Exit State Systems
            .add_systems(
                OnExit(AppState::Game),
                (despawn_enemies, despawn_notes_on_exit),
            );
    }
}

// Events
#[derive(Event)]
pub struct EnemyIsArrivingEvent(pub String);

#[derive(Event)]
pub struct MessageBoxRequest(Entity, String);

#[derive(Event)]
pub struct DoggyTheme;
