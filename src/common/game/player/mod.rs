use bevy::prelude::*;

// ───── Current Crate Imports ────────────────────────────────────────────── //

use self::systems::*;
use super::SimulationState;
use crate::common::{events::PlayerHit, AppState};

// ───── Submodules ───────────────────────────────────────────────────────── //

// Top-level modules
pub mod components;
pub mod systems;

// ───── Constants ────────────────────────────────────────────────────────── //

pub const PLAYER_SPEED: f32 = 25000.;
pub const LIVES_COUNT: u64 = 3;
pub const SPACESHIP_SIZE: f32 = 64.;
pub const DOG_SIZE: Vec2 = Vec2::new(156., 154.);

// ───── Body ─────────────────────────────────────────────────────────────── //

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum PlayerSystemSet {
    Movement,
}

#[derive(States, Clone, PartialEq, Eq, Default, Debug, Hash)]
pub enum PlayerState {
    #[default]
    Vulnerable,
    Invulnerable,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        #[cfg(not(target_arch = "wasm32"))]
        {
            app
                // Enter State Systems
                .add_system(spawn_player.in_schedule(OnEnter(AppState::Game)))
                // Systems
                .add_system(
                    player_movement
                        .in_set(PlayerSystemSet::Movement)
                        .in_set(OnUpdate(SimulationState::Running))
                        .in_set(OnUpdate(AppState::Game)),
                );
        }

        #[cfg(target_arch = "wasm32")]
        {
            app
                // Enter State Systems
                .add_system(
                    spawn_player_without_gpu_particles
                        .in_schedule(OnEnter(AppState::Game)),
                )
                // Systems
                .add_systems(
                    (
                        player_movement_without_gpu_particles,
                        poll_and_despawn_smoke_particles,
                    )
                        .in_set(PlayerSystemSet::Movement)
                        .in_set(OnUpdate(SimulationState::Running))
                        .in_set(OnUpdate(AppState::Game)),
                )
                // Exit State Systems
                .add_system(
                    despawn_smoke_particles_on_exit_state
                        .in_schedule(OnExit(AppState::Game)),
                );
        }

        app
            // Events
            .add_event::<PlayerHit>()
            // System Sets
            .configure_set(PlayerSystemSet::Movement)
            // States
            .add_state::<PlayerState>()
            // Systems
            .add_systems(
                (
                    handle_player_collision,
                    regenerate_player,
                    spawn_particles_on_collision_with_enemy,
                    poll_and_despawn_collision_particles,
                )
                    .in_set(OnUpdate(SimulationState::Running))
                    .in_set(OnUpdate(AppState::Game)),
            )
            .add_systems(
                (count_player_invulnerability_timer, blink_player)
                    .in_set(OnUpdate(SimulationState::Running))
                    .in_set(OnUpdate(AppState::Game))
                    .in_set(OnUpdate(PlayerState::Invulnerable)),
            )
            // Exit State Systems
            .add_systems(
                (
                    despawn_player_on_exit_game_state,
                    despawn_collision_particles,
                )
                    .in_schedule(OnExit(AppState::Game)),
            );
    }
}
