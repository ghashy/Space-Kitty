use bevy::prelude::*;

// ───── Current Crate Imports ────────────────────────────────────────────── //

use self::systems::*;
use super::SimulationState;
use crate::{events::PlayerHit, AppState};

// ───── Submodules ───────────────────────────────────────────────────────── //

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
            // Events
            .add_event::<PlayerHit>()
            // System Sets
            .configure_set(PlayerSystemSet::Movement)
            // States
            .add_state::<PlayerState>()
            // Enter State Systems
            .add_system(spawn_player.in_schedule(OnEnter(AppState::Game)))
            // Systems
            .add_system(
                player_movement
                    .in_set(PlayerSystemSet::Movement)
                    .in_set(OnUpdate(SimulationState::Running))
                    .in_set(OnUpdate(AppState::Game)),
            )
            .add_system(
                handle_player_collision
                    .in_set(OnUpdate(SimulationState::Running))
                    .in_set(OnUpdate(AppState::Game)), // .in_set(OnUpdate(PlayerState::Vulnerable)),
            )
            .add_systems(
                (count_player_invulnerability_timer, blink_player)
                    .in_set(OnUpdate(SimulationState::Running))
                    .in_set(OnUpdate(AppState::Game))
                    .in_set(OnUpdate(PlayerState::Invulnerable)),
            )
            // Exit State Systems
            .add_system(
                despawn_player_on_exit_game_state
                    .in_schedule(OnExit(AppState::Game)),
            );
    }
}
