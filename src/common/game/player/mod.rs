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
        app
            // Enter State Systems
            .add_systems(
                OnEnter(AppState::Game),
                spawn_player_without_gpu_particles,
            )
            // Systems
            .add_systems(
                Update,
                (
                    player_movement_without_gpu_particles,
                    poll_and_despawn_smoke_particles,
                )
                    .in_set(PlayerSystemSet::Movement)
                    .run_if(in_state(SimulationState::Running))
                    .run_if(in_state(AppState::Game)),
            )
            // Exit State Systems
            .add_systems(
                OnExit(AppState::Game),
                despawn_smoke_particles_on_exit_state,
            );

        app
            // Events
            .add_event::<PlayerHit>()
            // System Sets
            .configure_set(Update, PlayerSystemSet::Movement)
            // States
            .add_state::<PlayerState>()
            // Systems
            .add_systems(
                Update,
                (
                    handle_player_collision,
                    regenerate_player,
                    spawn_particles_on_collision_with_enemy,
                    poll_and_despawn_collision_particles,
                )
                    .run_if(in_state(SimulationState::Running))
                    .run_if(in_state(AppState::Game)),
            )
            .add_systems(
                Update,
                (count_player_invulnerability_timer, blink_player)
                    .run_if(in_state(SimulationState::Running))
                    .run_if(in_state(AppState::Game))
                    .run_if(in_state(PlayerState::Invulnerable)),
            )
            // Exit State Systems
            .add_systems(
                OnExit(AppState::Game),
                (
                    despawn_player_on_exit_game_state,
                    despawn_collision_particles,
                ),
            );
    }
}
