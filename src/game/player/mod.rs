use bevy::prelude::*;

// ───── Current Crate Imports ────────────────────────────────────────────── //

use self::systems::*;
use super::SimulationState;
use crate::{events::PlayerHit, AppState};

// ───── Submodules ───────────────────────────────────────────────────────── //

pub mod components;
pub mod systems;

// ───── Constants ────────────────────────────────────────────────────────── //

pub const PLAYER_SPEED: f32 = 50000.;
pub const BALL_SIZE: f32 = 64.;

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
            .add_systems(
                (
                    player_movement.in_set(PlayerSystemSet::Movement),
                    enemy_hit_player,
                    player_hit_star,
                )
                    .in_set(OnUpdate(SimulationState::Running))
                    .in_set(OnUpdate(AppState::Game))
                    .in_set(OnUpdate(PlayerState::Vulnerable)),
            )
            .add_systems(
                (
                    player_movement.in_set(PlayerSystemSet::Movement),
                    player_hit_star,
                    count_player_invulnerability_timer,
                    blink_player,
                )
                    .in_set(OnUpdate(SimulationState::Running))
                    .in_set(OnUpdate(AppState::Game))
                    .in_set(OnUpdate(PlayerState::Invulnerable)),
            )
            // Exit State Systems
            .add_system(despawn_player.in_schedule(OnExit(AppState::Game)));
    }
}
