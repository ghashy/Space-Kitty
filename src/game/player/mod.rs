use bevy::prelude::*;

// ----- Module ------------------------------------------------------------- //

pub mod components;
pub mod systems;

use crate::AppState;

use self::systems::*;

use super::SimulationState;

// ----- Constants ---------------------------------------------------------- //

pub const PLAYER_SPEED: f32 = 50000.;
pub const BALL_SIZE: f32 = 64.;

// ----- Body --------------------------------------------------------------- //

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum PlayerSystemSet {
    Movement,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            // System Sets
            .configure_set(PlayerSystemSet::Movement)
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
                    .in_set(OnUpdate(AppState::Game)),
            )
            // Exit State Systems
            .add_system(despawn_player.in_schedule(OnExit(AppState::Game)));
    }
}
