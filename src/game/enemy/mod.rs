use bevy::prelude::*;

// ----- Module ------------------------------------------------------------- //

pub mod components;
pub mod resources;
pub mod systems;

use crate::AppState;

use self::systems::*;

use super::SimulationState;

// ----- Constants ---------------------------------------------------------- //

const NUMBER_OF_ENEMIES: usize = 3;
const ENEMY_SPEED: f32 = 500.;

// ----- Body --------------------------------------------------------------- //

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app
            // Enter State Systems
            .add_system(spawn_enemies.in_schedule(OnEnter(AppState::Game)))
            // Systems
            .add_systems(
                (
                    enemy_movement,
                    update_enemy_direction,
                    confine_enemy_movement,
                    spawn_enemy_on_game_progress,
                )
                    .in_set(OnUpdate(AppState::Game))
                    .in_set(OnUpdate(SimulationState::Running)),
            )
            // Exit State Systems
            .add_system(despawn_enemies.in_schedule(OnExit(AppState::Game)));
    }
}
