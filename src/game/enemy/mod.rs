use bevy::prelude::*;

// ───── Current Crate Imports ────────────────────────────────────────────── //

use self::systems::*;
use super::SimulationState;
use crate::AppState;

// ───── Submodules ───────────────────────────────────────────────────────── //

pub mod components;
pub mod resources;
pub mod systems;

// ───── Constants ────────────────────────────────────────────────────────── //

const NUMBER_OF_ENEMIES_ON_START: usize = 3;
const ENEMY_SPEED: f32 = 10000.;

// ───── Body ─────────────────────────────────────────────────────────────── //

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app
            // Enter State Systems
            .add_system(spawn_enemies.in_schedule(OnEnter(AppState::Game)))
            // Systems
            .add_systems(
                (
                    update_enemy_direction,
                    enemy_movement,
                    spawn_enemy_on_game_progress,
                )
                    .in_set(OnUpdate(AppState::Game))
                    .in_set(OnUpdate(SimulationState::Running)),
            )
            // Exit State Systems
            .add_system(despawn_enemies.in_schedule(OnExit(AppState::Game)));
    }
}
